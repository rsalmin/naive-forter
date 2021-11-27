use std::io;
use std::io::prelude::*;
use std::str;

mod state;
mod stack;
mod dict;

use state::State;

macro_rules! next {
    ( $x:ident ) => {
        {
            let v : Option<&str> = $x.next();
            if v.is_none() { break; }
            v.unwrap()
        }
    }
}

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

        interpret(&mut state, &input_line);

        input_line.clear();
    }

    println!("Bye!");
    Ok(())
}

fn parse_num(str : &str) -> Option<u8> {
    str.parse::<u8>().ok()
}

fn interpret(state : &mut State, input_line: &str) {

    let mut iter = input_line.split_whitespace();

    loop {

        let part = next!( iter );     //break when None

        if part == ":" {
            let cmd_name = String::from( next!( iter ) );
            let cmd_body :String  = iter.by_ref().take_while(|&x| x != ";").map(|x| format!("{} ", x) ).collect::<String>();

            //let cmd = move |_s : &mut State| { println!("{}", cmd_body); };
            let cmd = move |s : &mut State| { interpret(s, &cmd_body); };

            state.dict.insert_closure(&cmd_name, Box::new(cmd));

            continue;
         }

        if let Some(cmd) = state.dict.get(part) {
            cmd(state);
            continue;
        }

        if let Some(n) = parse_num(part) {
            state.stack.push(n);
            continue
        }

        println!("{} ?", part);
    }

}