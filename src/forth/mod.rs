use crate::state::{State, Function, StackType};
use crate::input_stream::InputStream;
use std::str;
use std::rc::Rc;


fn parse_num(str : &str) -> Option<StackType> {
    str.parse::<StackType>().ok()
}

pub fn interpret(state : &mut State, input_stream: &mut InputStream) -> Result<(), String> {

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

         if part == "(" {
             let _ = input_stream.take_until(')').ok_or("not found )")?; // didn't care about parentheses balance
             continue;
        }

        if let Some(cmd) = state.dict.get(&part) {
            cmd(state, input_stream)?;
            continue;
        }

        if let Some(n) = parse_num(&part) {
            state.stack.push(n);
            continue
        }

        println!("{} ?", part);
    }

}

pub fn compile(state : &State, input_line: &str) -> Result<Function, String> {

    let mut input_stream = InputStream::from(input_line);

    let mut code : Vec<Function> = Vec::new();

    loop {

        let part = input_stream.next_token();
        if part.is_none() { break; }
        let part = part.unwrap();

         if part == ".\"" {
             let text = input_stream.take_until('"').ok_or("not found '\"'")?;
             code.push( Rc::new(Box::new( move |_s : &mut State, _i : &mut InputStream | { print!("{}", text); Ok(()) } )) );
             continue;
         }

         if part == "(" {
             let _ = input_stream.take_until(')').ok_or("not found )")?; // didn't care about parentheses balance
             continue;
        }

        if let Some(cmd) = state.dict.get(&part) {
            code.push( cmd );
            continue;
        }

        if let Some(n) = parse_num(&part) {
            code.push( Rc::new(Box::new( move |s : &mut State, _i : &mut InputStream | { s.stack.push(n); Ok(()) } )) );
            continue;
        }

        return Err( format!("{} ?", part) );
    }

    let cls = move |state : &mut State, input : &mut InputStream | {
        for cmd in code.iter() {
            cmd(state, input)?;
        }
        Ok(())
    };
    Ok( Rc::new(Box::new(cls)) )
}