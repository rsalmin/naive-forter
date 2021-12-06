use crate::dict::Dict;
use crate::state::State;
use crate::input_stream::InputStream;
use crate::forth::interpret;
use std::iter;
use std::rc::Rc;

pub fn populate_dict(d : &mut Dict) {
        d.insert_state_fn("CR", |_s : &mut State |  { println!(); Ok(()) } );
        d.insert_state_fn("SPACE", |_s : &mut State |  { print!(" "); Ok(()) } );
        d.insert_state_fn("SPACES", |s : &mut State |  {
            let n = s.stack.pop().ok_or("stack is empty for SPACES")?;
            print!("{}", iter::repeat(' ').take(n.unsigned_abs() as usize).collect::<String>() );
            Ok(())
        } );
        d.insert_state_fn("EMIT", |s : &mut State | {
            let c = s.stack.pop().ok_or("stack is empty for EMIT")? as u8;
            let c = c as char;
            print!("{}", c);
            Ok(())
        } );
        d.insert_state_fn(".", |s : &mut State | {
            let n = s.stack.pop().ok_or("stack is empty for .")?;
            print!("{}", n);
            Ok(())
        });
        d.insert_state_fn("+", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of +")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of +")?;
            s.stack.push(a + b);
            Ok(())
        });
        d.insert_state_fn("-", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of -")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of -")?;
            s.stack.push(a - b);
            Ok(())
        });
        d.insert_state_fn("*", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of *")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of *")?;
            s.stack.push(a * b);
            Ok(())
        });
        d.insert_state_fn("/", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of /")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of /")?;
            s.stack.push(a / b);
            Ok(())
        });
        d.insert_state_fn("/MOD", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of /MOD")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of /MOD")?;
            s.stack.push(a % b);
            s.stack.push(a / b);
            Ok(())
        });
        d.insert_state_fn("MOD", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of MOD")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of MOD")?;
            s.stack.push(a % b);
            Ok(())
        });
        d.insert_state_fn("SWAP", |s : &mut State | {
            s.stack.swap().ok_or("not enough data for SWAP".to_string())
        });
        d.insert_state_fn("DUP", |s : &mut State | {
            s.stack.dup().ok_or("not enough data for DUP".to_string())
        });
        d.insert_state_fn("OVER", |s : &mut State | {
            s.stack.over().ok_or("not enough data for OVER".to_string())
        });
        d.insert_state_fn("ROT", |s : &mut State | {
            s.stack.rot().ok_or("not enough data for ROT".to_string())
        });
        d.insert_state_fn("DROP", |s : &mut State | {
            s.stack.drop().ok_or("not enough data for DROP".to_string())
        });
        d.insert_state_fn(".S", |s : &mut State | {
            let s = s.stack.state();
            let sz = s.len();
            print!("<{}>", sz);
            for e in s.iter() {
                print!(" {}", e);
            }
            Ok(())
        });
        d.insert_state_fn("2SWAP", |s : &mut State | {
            s.stack.two_swap().ok_or("not enough data for 2SWAP".to_string())
        });
        d.insert_state_fn("2DUP", |s : &mut State | {
            s.stack.two_dup().ok_or("not enough data for 2DUP".to_string())
        });
        d.insert_state_fn("2OVER", |s : &mut State | {
            s.stack.two_over().ok_or("not enough data for 2OVER".to_string())
        });
        d.insert_state_fn("2DROP", |s : &mut State | {
            s.stack.two_drop().ok_or("not enough data for 2DROP".to_string())
        });
        d.insert_fn("FORGET", | s : &mut State, input : &mut InputStream | {
           let t = input.next_token().ok_or("no arg for FORGET")?;
           s.dict.forget(&t).ok_or(format!("no word {} in dictionary", t))
        });
        d.insert_fn("MARKER", | state : &mut State, input : &mut InputStream | {
           let t = input.next_token().ok_or("no arg for MARKER")?;
           let state_copy = state.clone();
           let cls = move |s : &mut State, _ : &mut InputStream | {
               *s = state_copy.clone(); //FnOnce without clone
               Ok(())
           };
           state.dict.insert_closure( &t, Rc::new(Box::new( cls  ) ) );
           Ok(())
        });

        d.insert_fn("INCLUDE", | state : &mut State, input : &mut InputStream | {
           use std::fs::File;
           use std::io::{BufReader, BufRead};

           let t = input.next_token().ok_or("no arg for INCLUDE")?;
           let file = File::open(t).map_err(|x| x.to_string())?;
           let reader = BufReader::new(file);
           for line in reader.lines() {
               let str = line.map_err(|x| x.to_string())?;
               let mut input = InputStream::from(&str);
               interpret(state, &mut input)?;
           }
           Ok(())
        });

}