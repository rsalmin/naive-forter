
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_pop_state() {
        let mut s = Stack::new();
        s.push(10);
        s.push(20);

        let state = s.state();
        assert!(state.len() == 2);
        assert!(state[0] == 10);
        assert!(state[1] == 20);
        assert!(Some(20)== s.pop());
        assert!(Some(10) == s.pop());
        assert!(None == s.pop());
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
}