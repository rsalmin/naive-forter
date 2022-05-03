use crate::state::State;
use crate::input_stream::InputStream;
use crate::output::Output;
use std::rc::Rc;

mod populate_dict;
pub use populate_dict::populate_dict;

pub type Error = String;
pub type CompiledFunction = Rc<Box<dyn Fn(&mut State, &mut InputStream) -> Result<Output, Error>>>;
pub type FunctionCompiler = Rc<Box<dyn Fn(&mut InputStream) -> Result<CompiledFunction, Error>>>;

#[derive(Clone)]
pub struct Dict {
    dict : Vec<(String, FunctionCompiler)>,
}

impl Dict {
    pub fn new() -> Dict {
        let mut d = Dict { dict : Vec::<(String, FunctionCompiler)>::new() };
        populate_dict(&mut d);
        d
    }

    pub fn get(&self, key : &str) -> Option<FunctionCompiler> {
        for x in self.dict.iter().rev() {
            if x.0 == key {
                return Some( x.1.clone() );
            }
        }
        None
    }

    pub fn insert_state_fn(&mut self, key : &str, f : fn(&mut State) -> Result<Output, String>) {
        let fc = move |state : &mut State, _ : &mut InputStream| {f(state)};
        let rf : CompiledFunction = Rc::new(Box::new(fc));
        self.dict.push((String::from(key), Rc::new(Box::new(
           move | _ : &mut InputStream| {
               Ok(rf.clone())
           }))));
    }

    pub fn insert_ret_closure(&mut self, key : &str, f : CompiledFunction) {
        let cls = move | _input : &mut InputStream | {
            Ok(f.clone())
        };
        self.dict.push((String::from(key), Rc::new(Box::new(cls))));
    }

    //pub fn insert_ret_closure2<T>(&mut self, key : &str, f : T)
    //    where T : for<'r> Fn(&'r mut State) -> Result<(), Error> {
    //    let f = Rc::new(Box::new(f));
   //     let cls = move | _input : &mut InputStream | {
    //        Ok(f.clone())
   //     };
   //     self.dict.push((String::from(key), Rc::new(Box::new(cls))));
   // }

    pub fn insert_closure(&mut self, key : &str, f : FunctionCompiler) {
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

    fn forth_state(str: &str) -> State {
        let mut s = State::new();
        let mut i = InputStream::from(str);
        interpret(&mut s, &mut i).unwrap();
        s
    }

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
    fn division() {
        let mut s = forth_state("10 2 /");
        assert_eq!(s.stack.pop(), Some(5));

        let mut s = State::new();
        let mut i = InputStream::from("10 0 /");
        assert!( interpret(&mut s, &mut i).is_err() );
    }

    #[test]
    fn marker() {
        let mut s = State::new();
        s.stack.push(1);

        let mut i = InputStream::from(" -mark");
        s.dict.get("MARKER").unwrap()(&mut i).unwrap()(&mut s, &mut i).unwrap();

        s.stack.push(2);
        let mut i = InputStream::from(": 4MORE 4 + ;");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.dict.get("4MORE").is_some());
        s.dict.get("-mark").unwrap()(&mut i).unwrap()(&mut s, &mut i).unwrap();

        assert!(s.dict.get("4MORE").is_none());
        assert!(s.stack.pop() == Some(1));
        assert!(s.stack.pop() == None);
    }

    #[test]
    fn comparision() {
        let mut s = State::new();

        let mut i = InputStream::from("4 5 =");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("5 5 =");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("4 5 <>");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("5 5 <>");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("4 5 <");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("5 5 <");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("4 5 >");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("55 5 >");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("5 0=");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("0 0=");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("5 0<");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("-5 0<");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("5 0>");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("-5 0>");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));
    }

    #[test]
    fn logical_operators() {
        let mut s = State::new();

        let mut i = InputStream::from("-1 -1 AND");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("-1 0 AND");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("0 -1 AND");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("0 0 AND");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("-1 -1 OR");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("-1 0 OR");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("0 -1 OR");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("0 0 OR");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));

        let mut i = InputStream::from("0 INVERT");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(-1));

        let mut i = InputStream::from("-1 INVERT");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(0));
    }

    #[test]
    fn if_then_word() {
        let mut s = State::new();
        let mut i = InputStream::from("10 10 = IF 15 THEN");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(15));

        let mut s = State::new();
        let mut i = InputStream::from("10 11 = IF 15 THEN");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == None);
    }

    #[test]
    fn if_else_then_word() {
        let mut s = State::new();
        let mut i = InputStream::from("10 10 = IF 15 ELSE 30 THEN");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(15));

        let mut s = State::new();
        let mut i = InputStream::from("10 11 = IF 15 ELSE 30 THEN");
        interpret(&mut s, &mut i).unwrap();
        assert!(s.stack.pop() == Some(30));
    }

    #[test]
    fn dup_word() {
        let mut s = forth_state("0 ?DUP");
        assert_eq!(s.stack.pop(), Some(0));
        assert_eq!(s.stack.pop(), None);

        let mut s = forth_state("10 ?DUP");
        assert_eq!(s.stack.pop(), Some(10));
        assert_eq!(s.stack.pop(), Some(10));
        assert_eq!(s.stack.pop(), None);
    }

    #[test]
    fn abort_word() {
        let mut s = forth_state("10 2 0 ABORT\" what \" /");
        assert_eq!(s.stack.pop(), Some(5));
        assert_eq!(s.stack.pop(), None);

        let mut s = forth_state("10 0 1 ABORT\" what \" /");
        assert_eq!(s.stack.pop(), None);
    }
}