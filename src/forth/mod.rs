use crate::state::{State, Dict, RetFunction, StackType};
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
            let cmd_body = input_stream.take_until_first(";").ok_or("not found ';'")?;
            let cmd =  compile(&state.dict, cmd_body)?;
            state.dict.insert_ret_closure(&cmd_name, cmd);
            continue;
         }

         if part == "(" {
             let _ = input_stream.take_until_first(")").ok_or("not found )")?; // didn't care about parentheses balance
             continue;
        }

        if let Some(cmd) = state.dict.get(&part) {
            cmd(input_stream)?(state)?;
            continue;
        }

        if let Some(n) = parse_num(&part) {
            state.stack.push(n);
            continue
        }

        println!("{} ?", part);
    }

}

pub fn compile(dict : &Dict, mut input_stream: InputStream) -> Result<RetFunction, String> {

    let mut code : Vec<RetFunction> = Vec::new();

    loop {

        let part = input_stream.next_token();
        if part.is_none() { break; }
        let part = part.unwrap();

         if part == "(" {
             let _ = input_stream.take_until_first(")").ok_or("not found )")?; // didn't care about parentheses balance
             continue;
        }

        if let Some(cmd) = dict.get(&part) {
            code.push( cmd(&mut input_stream)? );
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
