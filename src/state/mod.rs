use crate::stack::Stack;

pub use crate::dict::{Dict, FunctionCompiler, CompiledFunction};
pub use crate::stack::StackType;

#[derive(Clone)]
pub struct State {
    pub stack : Stack,
    pub dict : Dict,
}

impl State {
    pub fn new() -> State {
        State { stack : Stack::new(), dict : Dict::new()}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trivial() {
        let _s = State::new();
    }

}