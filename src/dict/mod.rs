use std::collections::HashMap;
use crate::state::State;
use std::rc::Rc;

mod populate_dict;
pub use populate_dict::populate_dict;

pub type Function = Rc<Box<dyn Fn(&mut State) -> Result<(), String>>>;

pub struct Dict {
    dict : HashMap<String, Function>,
}

impl Dict {
    pub fn new() -> Dict {
        let mut d = Dict { dict : HashMap::<String, Function>::new() };

        populate_dict(&mut d);

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