#[derive(Debug)]
pub struct State {
}

impl State {
    pub fn new() -> State {
        State {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trivial() {
        let s = State::new();
    }

}