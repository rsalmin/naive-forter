use std::io;
use std::io::prelude::*;
use std::str;
use std::rc::Rc;

mod state;
mod stack;
mod dict;
mod input_stream;

use state::{State, Function, StackType};
use input_stream::InputStream;

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

fn parse_num(str : &str) -> Option<StackType> {
    str.parse::<StackType>().ok()
}

fn interpret(state : &mut State, input_stream: &mut InputStream) -> Result<(), String> {

    loop {

        let part = input_stream.next_token();
        if part.is_none() { return Ok(()); }
        let part = part.unwrap();

        if part == ":" {
            let cmd_name =  input_stream.next_token().ok_or("token not found after :")?;
            let cmd_body = input_stream.take_until(';').ok_or("not found ';'")?;
            let cmd =  compile(state, &cmd_body)?;
            state.dict.insert_closure(&cmd_name, cmd);
            continue;
         }

         if part == ".\"" {
             let text = input_stream.take_until('"').ok_or("not found '\"'")?;
             print!("{}", text);
             continue;
         }

        if let Some(cmd) = state.dict.get(&part) {
            cmd(state)?;
            continue;
        }

        if let Some(n) = parse_num(&part) {
            state.stack.push(n);
            continue
        }

        println!("{} ?", part);
    }

}

fn compile(state : &State, input_line: &str) -> Result<Function, String> {

    let mut input_stream = InputStream::from(input_line);

    let mut code : Vec<Function> = Vec::new();

    loop {

        let part = input_stream.next_token();
        if part.is_none() { break; }
        let part = part.unwrap();

         if part == ".\"" {
             let text = input_stream.take_until('"').ok_or("not found '\"'")?;
             code.push( Rc::new(Box::new( move |_s : &mut State | { print!("{}", text); Ok(()) } )) );
             continue;
         }

        if let Some(cmd) = state.dict.get(&part) {
            code.push( cmd );
            continue;
        }

        if let Some(n) = parse_num(&part) {
            code.push( Rc::new(Box::new( move |s : &mut State | { s.stack.push(n); Ok(()) } )) );
            continue;
        }

        return Err( format!("{} ?", part) );
    }

    let cls = move |state : &mut State | {
        for cmd in code.iter() {
            cmd(state)?;
        }
        Ok(())
    };
    Ok( Rc::new(Box::new(cls)) )
}