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
            println!("{}", n);
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
}