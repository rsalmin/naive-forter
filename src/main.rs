use std::io;
use std::io::prelude::*;

mod state;
mod stack;
mod dict;

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

fn parse_num(str : &str) -> Option<u8> {
    str.parse::<u8>().ok()
}

fn process(state : &mut State, input_line : &str) {

    // let cmds : Vec<&str> = input_line.split_whitespace().collect();

    for part in input_line.split_whitespace() {

        if let Some(&cmd) = state.dict.get(part) {
            cmd(&mut state.stack);
            continue;
        }

        if let Some(n) = parse_num(part) {
            state.stack.push(n);
            continue
        }

        println!("No such command in dictonary: {}", part);
    }

}