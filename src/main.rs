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

    let mut state = State::new();

    let mut input_stream = InputStream::from("");

    let mut is_input_finished = true;

    loop {

        io::stdout().write( prompt.as_bytes() )?;
        io::stdout().flush()?;

        let mut input_line = String::new();
        let bytes = io::stdin().read_line(&mut input_line)?;
        if bytes == 0 {  //EOF
            println!();
            break;
        }

        let input_line_stream = InputStream::from(&input_line);

        let is_starting = input_line_stream.is_first_token_equal(":");
        let is_ending = input_line_stream.is_token_exists(";");

        input_stream.append( input_line_stream );

        // condition for staring multiline input
        if is_input_finished && is_starting && ! is_ending {
            is_input_finished = false;
            continue;
        }

        // condition for continuing multiline input
        if ! is_input_finished && ! is_ending {
            continue;
        }

        // otherwise input_stream is ready
        if let Err( err ) = interpret(&mut state, &mut input_stream) {
            println!("error : {}", err);
        }
        is_input_finished = true;
        input_stream.clear();

    }

    println!("Bye!");
    Ok(())
}
