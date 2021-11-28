use std::collections::HashMap;
use crate::state::State;
use std::iter;
use std::rc::Rc;

pub type Function = Rc<Box<dyn Fn(&mut State)>>;

pub struct Dict {
    dict : HashMap<String, Function>,
}

impl Dict {
    pub fn new() -> Dict {
        let mut d = Dict { dict : HashMap::<String, Function>::new() };

        d.insert_fn("Hi", |_s : &mut State | println!("*") );
        d.insert_fn("CR", |_s : &mut State | println!() );
        d.insert_fn("SPACES", |s : &mut State |  {
            let n = s.stack.pop();
            print!("{}", iter::repeat(' ').take(n.into()).collect::<String>() );
        } );
        d.insert_fn("EMIT", |s : &mut State | {
            let c = s.stack.pop() as char;
            print!("{}", c);
        } );

        d
    }

    pub fn get(&self, key : &str) -> Option<Function> {
        self.dict.get(key).map(|x| x.clone())
    }

    pub fn insert_fn(&mut self, key : &str, f : fn(&mut State)) {
        self.dict.insert(String::from(key), Rc::new(Box::new(f)));
    }

    pub fn insert_closure(&mut self, key : &str, f : Rc<Box<dyn Fn(&mut State)>>) {
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