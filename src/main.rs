use std::io;
use std::io::prelude::*;

mod state;
mod stack;
mod dict;
mod input_stream;
mod forth;

use state::State;
use input_stream::InputStream;
use forth::interpret;

fn main() -> io::Result<()> {

    let prompt = " > ";
    let mut input_line = String::new();

    let mut state = State::new();

    loop {

        io::stdout().write( prompt.as_bytes() )?;
        io::stdout().flush()?;

        let bytes = io::stdin().read_line(&mut input_line)?;
        if bytes == 0 {  //EOF
            println!();
            break;
        }

        let mut input_stream = InputStream::from(&input_line);
        if let Err( err ) = interpret(&mut state, &mut input_stream) {
            println!("error : {}", err);
        }

        input_line.clear();
    }

    println!("Bye!");
    Ok(())
}
