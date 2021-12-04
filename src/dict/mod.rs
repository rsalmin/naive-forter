use crate::state::State;
use crate::input_stream::InputStream;
use std::rc::Rc;

mod populate_dict;
pub use populate_dict::populate_dict;

pub type Function = Rc<Box<dyn Fn(&mut State, &mut InputStream) -> Result<(), String>>>;

pub struct Dict {
    //dict : HashMap<String, Function>,
    dict : Vec<(String, Function)>,
}

impl Dict {
    pub fn new() -> Dict {
        let mut d = Dict { dict : Vec::<(String, Function)>::new() };
        populate_dict(&mut d);
        d
    }

    pub fn get(&self, key : &str) -> Option<Function> {
        for x in self.dict.iter().rev() {
            if x.0 == key {
                return Some( x.1.clone() );
            }
        }
        None
    }

    pub fn insert_state_fn(&mut self, key : &str, f : fn(&mut State) -> Result<(), String>) {
        self.dict.push((String::from(key), Rc::new(Box::new(
           move |state : &mut State, _ : &mut InputStream| {
               f(state)
           }))));
    }

    pub fn insert_fn(&mut self, key : &str, f : fn(&mut State, &mut InputStream) -> Result<(), String>) {
        self.dict.push((String::from(key), Rc::new(Box::new(f))));
    }

    pub fn insert_closure(&mut self, key : &str, f : Function) {
        self.dict.push((String::from(key), f));
    }

    pub fn forget(&mut self, key : &str) -> Option<()> {
        let p = self.dict.iter().rposition(|x| x.0 == key );
        match p {
            Some( pos ) => { let _ = self.dict.remove( pos ); Some(()) },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn constuct_and_populate() {
        let d = Dict::new();
        assert!(d.get("SPACES").is_some());
        assert!(d.get("NONE").is_none());
    }

    #[test]
    fn forget() {
        let mut d = Dict::new();
        assert!(d.get("SPACES").is_some());
        assert!(d.forget("SPACES").is_some());
        assert!(d.get("SPACES").is_none());
    }
}