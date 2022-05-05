use crate::dict::Dict;
use crate::state::State;
use crate::input_stream::InputStream;
use crate::output::Output;
use crate::forth::interpret;
use std::iter;
use std::rc::Rc;

pub fn populate_dict(d : &mut Dict) {
        d.insert_state_fn("CR", |_s : &mut State |  { Ok(Output::from("\n")) } );
        d.insert_state_fn("SPACE", |_s : &mut State |  { Ok(Output::from(" ")) } );
        d.insert_state_fn("SPACES", |s : &mut State |  {
            let n = s.stack.pop().ok_or("stack is empty for SPACES")?;
            Ok(Output::from(format!("{}", iter::repeat(' ').take(n.unsigned_abs() as usize).collect::<String>() ) ) )
        } );
        d.insert_state_fn("EMIT", |s : &mut State | {
            let c = s.stack.pop().ok_or("stack is empty for EMIT")? as u8;
            let c = c as char;
            Ok(Output::from( format!("{}", c) ))
        } );
        d.insert_state_fn(".", |s : &mut State | {
            let n = s.stack.pop().ok_or("stack is empty for .")?;
            Ok(Output::from( format!("{}", n) ))
        });
        d.insert_state_fn("+", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of +")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of +")?;
            s.stack.push(a + b);
            Ok(Output::none())
        });
        d.insert_state_fn("-", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of -")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of -")?;
            s.stack.push(a - b);
            Ok(Output::none())
        });
        d.insert_state_fn("*", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of *")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of *")?;
            s.stack.push(a * b);
            Ok(Output::none())
        });
        d.insert_state_fn("/", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of /")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of /")?;
            if b == 0 {
                return Err(String::from("division by Zero"));
            }
            s.stack.push(a / b);
            Ok(Output::none())
        });
        d.insert_state_fn("/MOD", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of /MOD")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of /MOD")?;
            s.stack.push(a % b);
            s.stack.push(a / b);
            Ok(Output::none())
        });
        d.insert_state_fn("MOD", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of MOD")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of MOD")?;
            s.stack.push(a % b);
            Ok(Output::none())
        });
        d.insert_state_fn("SWAP", |s : &mut State | {
            s.stack.swap().map(|()| Output::none()).ok_or( "not enough data for SWAP".to_string() )
        });
        d.insert_state_fn("DUP", |s : &mut State | {
            s.stack.dup().map(|()| Output::none()).ok_or( "not enough data for DUP".to_string() )
        });
        d.insert_state_fn("OVER", |s : &mut State | {
            s.stack.over().map(|()| Output::none()).ok_or( "not enough data for OVER".to_string() )
        });
        d.insert_state_fn("ROT", |s : &mut State | {
            s.stack.rot().map(|()| Output::none()).ok_or( "not enough data for ROT".to_string() )
        });
        d.insert_state_fn("DROP", |s : &mut State | {
            s.stack.drop().map(|()| Output::none()).ok_or( "not enough data for DROP".to_string() )
        });
        d.insert_state_fn(".S", |s : &mut State | {
            let s = s.stack.state();
            let sz = s.len();
            let mut output = Output::from(format!("<{}>", sz));
            for e in s.iter() {
                output.append( Output::from( format!(" {}", e) ) );
            }
            Ok(output)
        });
        d.insert_state_fn("2SWAP", |s : &mut State | {
            s.stack.two_swap().map(|()| Output::none()).ok_or( "not enough data for 2SWAP".to_string() )
        });
        d.insert_state_fn("2DUP", |s : &mut State | {
            s.stack.two_dup().map(|()| Output::none()).ok_or( "not enough data for 2DUP".to_string() )
        });
        d.insert_state_fn("2OVER", |s : &mut State | {
            s.stack.two_over().map(|()| Output::none()).ok_or( "not enough data for 2OVER".to_string() )
        });
        d.insert_state_fn("2DROP", |s : &mut State | {
            s.stack.two_drop().map(|()| Output::none()).ok_or( "not enough data for 2OVER".to_string() )
        });
        d.insert_closure("FORGET", Rc::new(Box::new(
            | input : &mut InputStream | {
                let t = input.next_token().ok_or("no arg for FORGET")?;
                let cls = move |s : &mut State, _ : &mut InputStream| {
                        s.dict.forget(&t).map(|()| Output::none()).ok_or(format!("no word {} in dictionary", t))
                };
                Ok(Rc::new(Box::new(cls)))
            })));
        d.insert_closure("MARKER", Rc::new(Box::new(
            | input : &mut InputStream | {
                let t = input.next_token().ok_or("no arg for MARKER")?;
                let ret_cls  = move |state : &mut State, _ : &mut InputStream| {
                    let state_copy = state.clone();
                    let cls = move |s : &mut State, _ : &mut InputStream | {
                        *s = state_copy.clone(); //FnOnce without clone
                        Ok(Output::none())
                    };
                    state.dict.insert_ret_closure( &t, Rc::new(Box::new( cls  ) ) );
                    Ok(Output::none())
                };
                Ok(Rc::new(Box::new(ret_cls)))
           })));

        d.insert_closure("INCLUDE", Rc::new(Box::new(
            | input : &mut InputStream | {
                let t = input.next_token().ok_or("no arg for INCLUDE")?;
                //FIXME: whole INCLUDE behaves like one command, not by many in respect to InputStream!
                let cls = move |state: &mut State, _: &mut InputStream| {
                    use std::fs::File;
                    use std::io::{BufReader, BufRead};

                    let file = File::open(t.clone()).map_err(|x| x.to_string())?;
                    let reader = BufReader::new(file);
                    let mut output = Output::none();
                    for line in reader.lines() {
                        let str = line.map_err(|x| x.to_string())?;
                        let mut input = InputStream::from(&str);
                        output.append( interpret(state, &mut input)? );
                    }
                    Ok(output)
                };
                Ok(Rc::new(Box::new(cls)))
             })));

        d.insert_closure(".\"", Rc::new(Box::new(
            | input : &mut InputStream | {
                let text = input.take_until_first("\"").ok_or("not found '\"'")?;
                let cls = move |_: &mut State, _: &mut InputStream| {
                    Ok(Output::from( format!("{}", text) ))
                };
                Ok(Rc::new(Box::new(cls)))
             })));
        d.insert_state_fn("=", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of =")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of =")?;
            let r = if a == b { -1 } else { 0 };
            s.stack.push( r );
            Ok(Output::none())
        });
        d.insert_state_fn("<>", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of <>")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of <>")?;
            let r = if a != b { -1 } else { 0 };
            s.stack.push( r );
            Ok(Output::none())
        });
        d.insert_state_fn("<", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of <")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of <")?;
            let r = if a < b { -1 } else { 0 };
            s.stack.push( r );
            Ok(Output::none())
        });
        d.insert_state_fn(">", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of >")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of >")?;
            let r = if a > b { -1 } else { 0 };
            s.stack.push( r );
            Ok(Output::none())
        });
        d.insert_state_fn("0=", |s : &mut State | {
            let a = s.stack.pop().ok_or("no arg for 0=")?;
            let r = if a == 0 { -1 } else { 0 };
            s.stack.push( r );
            Ok(Output::none())
        });
        d.insert_state_fn("0<", |s : &mut State | {
            let a = s.stack.pop().ok_or("no arg for 0<")?;
            let r = if a < 0 { -1 } else { 0 };
            s.stack.push( r );
            Ok(Output::none())
        });
        d.insert_state_fn("0>", |s : &mut State | {
            let a = s.stack.pop().ok_or("no arg for 0>")?;
            let r = if a > 0 { -1 } else { 0 };
            s.stack.push( r );
            Ok(Output::none())
        });
        d.insert_state_fn("AND", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of AND")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of AND")?;
            let c = ( a != 0 ) && ( b != 0) ;
            s.stack.push( if c { -1 } else { 0 } );
            Ok(Output::none())
        });
        d.insert_state_fn("OR", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of OR")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of OR")?;
            let c = ( a != 0) || ( b != 0) ;
            s.stack.push( if c { -1 } else { 0 } );
            Ok(Output::none())
        });
        d.insert_state_fn("INVERT", |s : &mut State | {
            let a = s.stack.pop().ok_or("stack is empty for arg of INVERT")?;
            s.stack.push( ! a );
            Ok(Output::none())
        });

        d.insert_closure("IF", Rc::new(Box::new(
            | input : &mut InputStream | {
                let else_cls_stream =  input.take_until_first("ELSE");
                let then_cls_stream = input.take_until_last("THEN").ok_or("not found THEN")?;

                let (true_cls_stream, false_cls_stream) =  if else_cls_stream.is_some() {
                    (else_cls_stream.unwrap(), Some(then_cls_stream) )
                } else {
                    (then_cls_stream, else_cls_stream)
                };

                let cls = move |s: &mut State, _: &mut InputStream| {
                    let a = s.stack.pop().ok_or("stack is empty for IF")?;
                    let output = if a != 0 {
                        let mut ics = true_cls_stream.clone();
                        interpret(s, &mut ics)?
                    } else if false_cls_stream.is_some() {
                        let mut ics = false_cls_stream.as_ref().unwrap().clone();
                        interpret(s, &mut ics)?
                    } else {
                        Output::none()
                    };
                    Ok(output)
                };
                Ok(Rc::new(Box::new(cls)))
             })));

        d.insert_state_fn("?DUP", |s : &mut State | {
            match s.stack.peek() {
                Some(&v) => {
                    if v != 0 {
                        s.stack.dup();
                     };
                    Ok(Output::none())
                },
                None => Err( "stack is empty for ?DUP".to_string() ),
            }
        });

        d.insert_closure("ABORT\"", Rc::new(Box::new(
            | input : &mut InputStream | {
                let text = input.take_until_first("\"").ok_or("not found '\"'")?;
                let cls = move |state: &mut State, input_stream: &mut InputStream| {
                    let v = state.stack.pop().ok_or("stack is emtpy for ABORT\"")?;
                    if v != 0 {
                        state.stack.clear();
                        input_stream.clear();
                        Ok(Output::from( format!("ABORT: {}", text) ))
                    } else {
                        Ok(Output::none())
                    }
                };
                Ok(Rc::new(Box::new(cls)))
          })));
        d.insert_state_fn("ABS", |s : &mut State | {
            let a = s.stack.pop().ok_or("stack is empty for ABS")?;
            s.stack.push( a.abs() );
            Ok(Output::none())
        });
}
