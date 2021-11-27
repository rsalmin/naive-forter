pub struct Stack {
    data : Vec<u8>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data : Vec::new() }
    }

    pub fn push(&mut self, v : u8) {
        self.data.push(v);
    }

    pub fn pop(&mut self) -> u8 {
        self.data.pop().expect("to much pop")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_pop() {
        let mut s = Stack::new();
        s.push(10);
        s.push(20);
        assert!(20 == s.pop());
        assert!(10 == s.pop());
    }
}