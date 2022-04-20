use std::fmt;

/// Output for Forth

#[derive(PartialEq)]
pub struct Output {
    data : Option<String>,
}


impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some( txt ) => write!(f, "Output is {}", txt),
            None => write!(f, "None Output"),
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some( v ) = &self.data {
            write!(f, "{} ", v)?;
        }
        Ok(())
    }
}

impl From<String> for Output {
    fn from(v : String) -> Self {
        Output { data : Some(v) }
    }
}

impl From<&str> for Output {
    fn from(v : &str) -> Output {
        Output { data : Some(String::from(v)) }
    }
}

impl Output {
    pub fn none() -> Output {
        Output { data : None }
    }

    pub fn append(&mut self, other : Output) {
        if other.data.is_none() {
            return;
        }

        if self.data.is_none() {
            self.data = other.data;
            return;
        }

        // TODO : write me properly
        let mut a = self.data.as_ref().unwrap().to_string();
        a.push_str( &other.data.unwrap() );

        self.data = Some( a );
     }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn append() {
        let mut a = Output::none();
        let b = Output::none();
        a.append(b);
        assert_eq!(a, Output::none());

        let mut a = Output::none();
        let b = Output::from("Hi");
        a.append(b);
        assert_eq!(a, Output::from("Hi"));

        let mut a = Output::from("Hi");
        let b = Output::none();
        a.append(b);
        assert_eq!(a, Output::from("Hi"));

        let mut a = Output::from("Hi ");
        let b = Output::from("Bro!");
        a.append(b);
        assert_eq!(a, Output::from("Hi Bro!"));
    }
}
