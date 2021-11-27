use crate::stack::Stack;
use crate::dict::Dict;

pub use crate::dict::Function;

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