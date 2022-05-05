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

    fn forth_check_stack(str: &str, vv : Vec<crate::stack::StackType>)  {
        let mut s = forth_state(str);
        for v in vv {
          let b = s.stack.pop();
          assert!(b.is_some(), "expect {} but stack is empty", v);
          let b = b.unwrap();
          assert_eq!(v, b, "expect {} but got {} at stack", v, b);
        }
        assert!(s.stack.pop().is_none(), "expecting empty stack but got something");
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
        forth_check_stack("4 5 =", vec![0] );
        forth_check_stack("5 5 =", vec![-1] );

        forth_check_stack("4 5 <>", vec![-1] );
        forth_check_stack("5 5 <>", vec![0] );
        forth_check_stack("4 5 <", vec![-1] );
        forth_check_stack("5 5 <", vec![0] );

        forth_check_stack("4 5 >", vec![0] );
        forth_check_stack("55 5 >", vec![-1] );

        forth_check_stack("5 0=", vec![0] );
        forth_check_stack("0 0=", vec![-1] );

         forth_check_stack("5 0<", vec![0]);
         forth_check_stack("-5 0<", vec![-1]);
         forth_check_stack("5 0>", vec![-1]);
         forth_check_stack("-5 0>", vec![0]);
    }

    #[test]
    fn logical_operators() {
        forth_check_stack("-1 -1 AND", vec![-1]);
        forth_check_stack("-1 0 AND", vec![0]);
        forth_check_stack("0 -1 AND", vec![0]);
        forth_check_stack("0 0 AND", vec![0]);

        forth_check_stack("-1 -1 OR", vec![-1]);
        forth_check_stack("-1 0 OR", vec![-1]);
        forth_check_stack("0 -1 OR", vec![-1]);
        forth_check_stack("0 0 OR", vec![0]);

        forth_check_stack("0 INVERT", vec![-1]);
        forth_check_stack("-1 INVERT", vec![0] );
     }

    #[test]
    fn if_then_word() {
        forth_check_stack("10 10 = IF 15 THEN", vec![15] );
        forth_check_stack("10 11 = IF 15 THEN", vec![]);
    }

    #[test]
    fn if_else_then_word() {
        forth_check_stack("10 10 = IF 15 ELSE 30 THEN", vec![15]);
        forth_check_stack("10 11 = IF 15 ELSE 30 THEN", vec![30] );
    }

    #[test]
    fn dup_word() {
        forth_check_stack("0 ?DUP", vec![0]);
        forth_check_stack("10 ?DUP", vec![10, 10]);
    }

    #[test]
    fn abort_word() {
        forth_check_stack("10 2 0 ABORT\" what \" /", vec![5]);
        forth_check_stack("10 0 1 ABORT\" what \" /", vec![]);
    }

    #[test]
    fn abs_word() {
        forth_check_stack("-10 ABS", vec![10]);
        forth_check_stack("10 ABS", vec![10]);
        forth_check_stack("0 ABS", vec![0]);
    }
}