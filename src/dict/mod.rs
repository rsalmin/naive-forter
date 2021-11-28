use std::collections::HashMap;
use crate::state::State;
use std::iter;
use std::rc::Rc;

pub type Function = Rc<Box<dyn Fn(&mut State) -> Result<(), String>>>;

pub struct Dict {
    dict : HashMap<String, Function>,
}

impl Dict {
    pub fn new() -> Dict {
        let mut d = Dict { dict : HashMap::<String, Function>::new() };

        d.insert_fn("CR", |_s : &mut State |  { println!(); Ok(()) } );
        d.insert_fn("SPACES", |s : &mut State |  {
            let n = s.stack.pop().ok_or("stack is empty")?;
            print!("{}", iter::repeat(' ').take(n.unsigned_abs().into()).collect::<String>() );
            Ok(())
        } );
        d.insert_fn("EMIT", |s : &mut State | {
            let c = s.stack.pop().ok_or("stack is empty")? as u8;
            let c = c as char;
            print!("{}", c);
            Ok(())
        } );
        d.insert_fn(".", |s : &mut State | {
            let n = s.stack.pop().ok_or("stack is empty")?;
            println!("{}", n);
            Ok(())
        });
        d.insert_fn("+", |s : &mut State | {
            let a = s.stack.pop().ok_or("stack is emtpy")?;
            let b = s.stack.pop().ok_or("stack is empty")?;
            s.stack.push(a + b);
            Ok(())
        });

        d
    }

    pub fn get(&self, key : &str) -> Option<Function> {
        self.dict.get(key).map(|x| x.clone())
    }

    pub fn insert_fn(&mut self, key : &str, f : fn(&mut State) -> Result<(), String>) {
        self.dict.insert(String::from(key), Rc::new(Box::new(f)));
    }

    pub fn insert_closure(&mut self, key : &str, f : Function) {
        self.dict.insert(String::from(key), f);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constuct() {
        let d = Dict::new();
        assert!(d.get("SPACES").is_some());
        assert!(d.get("NONE").is_none());
    }
}