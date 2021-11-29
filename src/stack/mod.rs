
pub type StackType = i32;

pub struct Stack {
    data : Vec<StackType>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { data : Vec::new() }
    }

    pub fn push(&mut self, v : StackType) {
        self.data.push(v);
    }

    pub fn pop(&mut self) -> Option<StackType> {
        self.data.pop()
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
        assert!(Some(20)== s.pop());
        assert!(Some(10) == s.pop());
        assert!(None == s.pop());
    }
}