use std::collections::HashMap;
use crate::stack::Stack;
use std::iter;

pub struct Dict {
    dict : HashMap<&'static str, fn(&mut Stack)>,
}

impl Dict {
    pub fn new() -> Dict {
        let mut d = HashMap::<&'static str, fn(&mut Stack)>::new();

        d.insert("Hi", |_s : &mut Stack| println!("*") );
        d.insert("Bro", |_s : &mut Stack| println!("+++") );
        d.insert("SPACES", |s : &mut Stack| { let n = s.pop(); print!("{}", iter::repeat(' ').take(n.into()).collect::<String>() )} );
        d.insert("EMIT", |s : &mut Stack| { let c = s.pop() as char; print!("{}", c); } );

        Dict { dict : d }
    }

    pub fn get(&self, key : &str) -> Option<&for<'r> fn(&'r mut Stack)> {
        self.dict.get(key)
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