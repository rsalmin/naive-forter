use crate::state::{State, StackType};
use crate::input_stream::InputStream;
use crate::output::Output;
use std::str;
use std::rc::Rc;


fn parse_num(str : &str) -> Option<StackType> {
    str.parse::<StackType>().ok()
}

pub fn interpret(state : &mut State, input_stream: &mut InputStream) -> Result<Output, String> {

    let mut output = Output::none();

    loop {

        let part = input_stream.next_token();
        if part.is_none() { return Ok(output); }
        let part = part.unwrap();

        if part == ":" {
            let cmd_name =  input_stream.next_token().ok_or("token not found after :")?;
            let cmd_body = input_stream.take_until_first(";").ok_or("not found ';'")?;
            let cmd =  move | _ : &mut State, i : &mut InputStream | {
                i.prepend( cmd_body.clone() );
                Ok(Output::none())
            };

            state.dict.insert_ret_closure(&cmd_name, Rc::new(Box::new(cmd)));
            continue;
         }

         if part == "(" {
             let _ = input_stream.take_until_first(")").ok_or("not found )")?; // didn't care about parentheses balance
             continue;
        }

        if let Some(cmd) = state.dict.get(&part) {
            output.append( cmd(input_stream)?(state, input_stream)? );
            continue;
        }

        if let Some(n) = parse_num(&part) {
            state.stack.push(n);
            continue
        }

        return Err(format!("{} ?", part));
    }

}
