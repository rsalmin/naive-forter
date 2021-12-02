use crate::dict::Dict;
use crate::state::State;
use std::iter;

pub fn populate_dict(d : &mut Dict) {
        d.insert_fn("CR", |_s : &mut State |  { println!(); Ok(()) } );
        d.insert_fn("SPACE", |_s : &mut State |  { print!(" "); Ok(()) } );
        d.insert_fn("SPACES", |s : &mut State |  {
            let n = s.stack.pop().ok_or("stack is empty for SPACES")?;
            print!("{}", iter::repeat(' ').take(n.unsigned_abs() as usize).collect::<String>() );
            Ok(())
        } );
        d.insert_fn("EMIT", |s : &mut State | {
            let c = s.stack.pop().ok_or("stack is empty for EMIT")? as u8;
            let c = c as char;
            print!("{}", c);
            Ok(())
        } );
        d.insert_fn(".", |s : &mut State | {
            let n = s.stack.pop().ok_or("stack is empty for .")?;
            print!("{}", n);
            Ok(())
        });
        d.insert_fn("+", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of +")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of +")?;
            s.stack.push(a + b);
            Ok(())
        });
        d.insert_fn("-", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of -")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of -")?;
            s.stack.push(a - b);
            Ok(())
        });
        d.insert_fn("*", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of *")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of *")?;
            s.stack.push(a * b);
            Ok(())
        });
        d.insert_fn("/", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of /")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of /")?;
            s.stack.push(a / b);
            Ok(())
        });
        d.insert_fn("/MOD", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of /MOD")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of /MOD")?;
            s.stack.push(a % b);
            s.stack.push(a / b);
            Ok(())
        });
        d.insert_fn("MOD", |s : &mut State | {
            let b = s.stack.pop().ok_or("stack is emtpy for second arg of MOD")?;
            let a = s.stack.pop().ok_or("stack is empty for first arg of MOD")?;
            s.stack.push(a % b);
            Ok(())
        });
        d.insert_fn("SWAP", |s : &mut State | {
            s.stack.swap().ok_or("not enough data for SWAP".to_string())
        });
        d.insert_fn("DUP", |s : &mut State | {
            s.stack.dup().ok_or("not enough data for DUP".to_string())
        });
        d.insert_fn("OVER", |s : &mut State | {
            s.stack.over().ok_or("not enough data for OVER".to_string())
        });
        d.insert_fn("ROT", |s : &mut State | {
            s.stack.rot().ok_or("not enough data for ROT".to_string())
        });
        d.insert_fn("DROP", |s : &mut State | {
            s.stack.drop().ok_or("not enough data for DROP".to_string())
        });
        d.insert_fn(".S", |s : &mut State | {
            let s = s.stack.state();
            let sz = s.len();
            print!("<{}>", sz);
            for e in s.iter() {
                print!(" {}", e);
            }
            Ok(())
        });
        d.insert_fn("2SWAP", |s : &mut State | {
            s.stack.two_swap().ok_or("not enough data for 2SWAP".to_string())
        });
        d.insert_fn("2DUP", |s : &mut State | {
            s.stack.two_dup().ok_or("not enough data for 2DUP".to_string())
        });
        d.insert_fn("2OVER", |s : &mut State | {
            s.stack.two_over().ok_or("not enough data for 2OVER".to_string())
        });
        d.insert_fn("2DROP", |s : &mut State | {
            s.stack.two_drop().ok_or("not enough data for 2DROP".to_string())
        });
}