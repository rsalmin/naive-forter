
pub type StackType = i32;

#[derive(Clone)]
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

    pub fn peek(&self) -> Option<&StackType> {
        self.data.last()
    }

    pub fn state(&self) -> Vec<StackType> {
        self.data.clone()
    }

    pub fn swap(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 2 {
            return None;
        }
        let a = self.data[s - 1];
        self.data[s - 1] = self.data[s - 2];
        self.data[s - 2] = a;
       Some(())
    }
    pub fn dup(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 1 {
            return None;
        }
        let a = self.data[s - 1];
        self.data.push(a);
        Some(())
    }
    pub fn over(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 2 {
            return None;
        }
        let a = self.data[s - 2];
        self.data.push(a);
        Some(())
    }
    pub fn rot(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 3 {
            return None;
        }
        let a = self.data[s - 3];
        self.data[s - 3] = self.data[s - 2];
        self.data[s - 2] = self.data[s - 1];
        self.data[s - 1] = a;
        Some(())
    }
    pub fn drop(&mut self) -> Option<()> {
        self.data.pop().map(|_|())
    }

    pub fn two_swap(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 4 {
            return None;
        }
        let a2 = self.data[s - 1];
        let a1 = self.data[s - 2];
        self.data[s - 1] = self.data[s - 3];
        self.data[s - 2] = self.data[s - 4];
        self.data[s - 3] = a2;
        self.data[s - 4] = a1;
       Some(())
    }

    pub fn two_dup(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 2 {
            return None;
        }
        let a2 = self.data[s - 1];
        let a1 = self.data[s - 2];
        self.data.push(a1);
        self.data.push(a2);
        Some(())
    }

    pub fn two_over(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 4 {
            return None;
        }
        let a2 = self.data[s - 3];
        let a1 = self.data[s - 4];
        self.data.push(a1);
        self.data.push(a2);
        Some(())
    }

    pub fn two_drop(&mut self) -> Option<()> {
        let s = self.data.len();
        if s < 2 {
            return None;
        }
        let _ = self.data.pop();
        self.data.pop().map(|_|())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

}

#[cfg(test)]
mod test {
    use super::*;


    impl Stack {
        fn len(&self) -> usize {
            self.data.len()
        }
    }


    #[test]
    fn push_pop_peek_state() {
        let mut s = Stack::new();
        s.push(10);
        s.push(20);
        assert_eq!(s.peek(), Some(&20));

        let state = s.state();
        assert_eq!(state.len(), 2);
        assert_eq!(state[0], 10);
        assert_eq!(state[1], 20);
        assert_eq!(Some(20), s.pop());
        assert_eq!(s.peek(), Some(&10));
        assert_eq!(Some(10), s.pop());
        assert_eq!(s.peek(), None);
        assert_eq!(None, s.pop());
    }

    #[test]
    fn swap() {
        let mut s = Stack::new();
        s.push(1);
        assert!(s.swap() == None);
        s.push(2);
        assert!(s.swap() == Some(()));
        assert!(Some(1)== s.pop());
        assert!(Some(2) == s.pop());
        assert!(None == s.pop());
    }

    #[test]
    fn dup() {
        let mut s = Stack::new();
        assert!(s.dup() == None);
        s.push(1);
        assert!(s.dup() == Some(()));
        assert!(Some(1)== s.pop());
        assert!(Some(1) == s.pop());
        assert!(None == s.pop());
    }

    #[test]
    fn over() {
        let mut s = Stack::new();
        assert!(s.over() == None);
        s.push(1);
        assert!(s.over() == None);
        s.push(2);
        assert!(s.over() == Some(()));
        assert!(Some(1)== s.pop());
        assert!(Some(2) == s.pop());
        assert!(Some(1)== s.pop());
        assert!(None == s.pop());
    }

    #[test]
    fn rot() {
        let mut s = Stack::new();
        assert!(s.rot() == None);
        s.push(1);
        assert!(s.rot() == None);
        s.push(2);
        assert!(s.rot() == None);
        s.push(3);
        assert!(s.rot() == Some(()));
        assert!(Some(1)== s.pop());
        assert!(Some(3) == s.pop());
        assert!(Some(2)== s.pop());
        assert!(None == s.pop());
    }

    #[test]
    fn drop() {
        let mut s = Stack::new();
        assert!(s.drop() == None);
        s.push(1);
        assert!(s.drop() == Some(()));
        assert!(None == s.pop());
    }

    #[test]
    fn two_swap() {
        let mut s = Stack::new();
        s.push(1);
        assert!(s.two_swap() == None);
        s.push(2);
        assert!(s.two_swap() == None);
        s.push(3);
        assert!(s.two_swap() == None);
        s.push(4);
        assert!(s.two_swap() == Some(()));
        assert!(Some(2)== s.pop());
        assert!(Some(1) == s.pop());
        assert!(Some(4)== s.pop());
        assert!(Some(3) == s.pop());
        assert!(None == s.pop());
    }

    #[test]
    fn two_dup() {
        let mut s = Stack::new();
        assert!(s.two_dup() == None);
        s.push(1);
        assert!(s.two_dup() == None);
        s.push(2);
        assert!(s.two_dup() == Some(()));
        assert!(Some(2)== s.pop());
        assert!(Some(1) == s.pop());
        assert!(Some(2)== s.pop());
        assert!(Some(1) == s.pop());
        assert!(None == s.pop());
    }

    #[test]
    fn two_over() {
        let mut s = Stack::new();
        s.push(1);
        assert!(s.two_over() == None);
        s.push(2);
        assert!(s.two_over() == None);
        s.push(3);
        assert!(s.two_over() == None);
        s.push(4);
        assert!(s.two_over() == Some(()));
        assert!(Some(2)== s.pop());
        assert!(Some(1) == s.pop());
        assert!(Some(4)== s.pop());
        assert!(Some(3) == s.pop());
        assert!(Some(2)== s.pop());
        assert!(Some(1) == s.pop());
        assert!(None == s.pop());
    }

    #[test]
    fn two_drop() {
        let mut s = Stack::new();
        assert!(s.two_drop() == None);
        s.push(1);
        assert!(s.two_drop() == None);
        s.push(2);
        assert!(s.two_drop() == Some(()));
        assert!(None == s.pop());
    }

    #[test]
    fn clear() {
        let mut s = Stack::new();
        for i in vec![1,2,3,4,5] {
            s.push(i);
        }
        assert_eq!(s.len(), 5);
        s.clear();
        assert_eq!(s.len(), 0);
    }

}