use std::io;
use std::io::prelude::*;

mod state;
use state::State;

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

        process(&mut state, &input_line);

        input_line.clear();
    }

    println!("Bye!");
    Ok(())
}

fn process(state : &mut State, input_line : &str) {

    for part in input_line.split_whitespace() {
        println!("Your part: {}", part);
    }

}