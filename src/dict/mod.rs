use crate::state::State;
use crate::input_stream::InputStream;
use std::rc::Rc;

mod populate_dict;
pub use populate_dict::populate_dict;

pub type Error = String;
pub type RetFunction = Rc<Box<dyn Fn(&mut State) -> Result<(), Error>>>;
pub type Function = Rc<Box<dyn Fn(&mut InputStream) -> Result<RetFunction, Error>>>;

#[derive(Clone)]
pub struct Dict {
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
        let rf : RetFunction = Rc::new(Box::new(f));
        self.dict.push((String::from(key), Rc::new(Box::new(
           move | _ : &mut InputStream| {
               Ok(rf.clone())
           }))));
    }

    pub fn insert_ret_closure(&mut self, key : &str, f : RetFunction) {
        let cls = move | _input : &mut InputStream | {
            Ok(f.clone())
        };
        self.dict.push((String::from(key), Rc::new(Box::new(cls))));
    }

    pub fn insert_closure(&mut self, key : &str, f : Function) {
        self.dict.push((String::from(key), f));
    }

    pub fn forget(&mut self, key : &str) -> Option<()> {
        loop {
            match self.dict.pop() {
                None => return None,
                Some( (k, _) ) => {
                    if k == key {
                        return Some(());
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::State;
    use crate::input_stream::InputStream;
    use crate::forth::interpret;
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

    #[test]
    fn marker() {
        let mut s = State::new();
        s.stack.push(1);

        let mut i = InputStream::from(" -mark");
        s.dict.get("MARKER").unwrap()(&mut i).unwrap()(&mut s).unwrap();

        s.stack.push(2);
        let mut i = InputStream::from(": 4MORE 4 + ;");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.dict.get("4MORE").is_some());
        s.dict.get("-mark").unwrap()(&mut i).unwrap()(&mut s).unwrap();

        assert!(s.dict.get("4MORE").is_none());
        assert!(s.stack.pop() == Some(1));
        assert!(s.stack.pop() == None);
    }

}