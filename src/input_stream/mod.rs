use std::collections::VecDeque;
use std::fmt;

/// Input Stream for Forth

pub struct InputStream {
    tokens : VecDeque<String>,
}

impl PartialEq for InputStream {
    fn eq(&self, other : &Self) -> bool {
        self.tokens == other.tokens
    }
}

impl fmt::Debug for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InputStream {:?}", self.tokens)
    }
}

impl fmt::Display for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for t in self.tokens.iter() {
            write!(f, "{} ", t)?;
        }
        Ok(())
    }
}

impl InputStream {
    pub fn from(s : &str) -> InputStream {
        let tokens = s.split_whitespace().map(|x| String::from(x)).collect::<VecDeque<String>>();
        InputStream { tokens }
    }

    /// returns next Forth token if present, all leading whitespaces(utf8) are consumed
    /// if none token can be found, returns None
    /// consumes one whitespaces (if any) after the each token - who cares?

    // TODO: fix copying
    pub fn next_token(&mut self) -> Option<String> {
       self.tokens.pop_front()
    }

    /// returns some string from current stream position till the occurence of `word`
    /// `word` must be surrounded by spaces
    /// if end word (utf8) can't be found returns None
    /// `word` not returned and consumed

    pub fn take_until(&mut self, word : &str) -> Option<InputStream> {
         if let Some( n ) = self.tokens.iter().position(|x| x == word) {
            let mut tokens = self.tokens.split_off(n); // keeps n elements, returns last
            std::mem::swap(&mut self.tokens, &mut tokens);
            self.tokens.pop_front(); // consume input word
            let r = InputStream { tokens };
            return Some(r);
        }

        None
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_until() {
        let mut input = InputStream::from(" : START 42 EMIT ; ");
        let part = input.take_until(";");
        assert_eq!(part, Some(InputStream::from(" : START 42 EMIT ")));
        let part = input.take_until(";");
        assert!(part.is_none());

        let mut input = InputStream::from(" 42 THEN");
        let part = input.take_until("THEN");
        assert_eq!(part, Some(InputStream::from(" 42 ")));

        let part = input.take_until("THEN");
        assert!(part.is_none());
    }

    #[test]
    fn next_token() {
        let mut input = InputStream::from("     : START 42 EMIT ;");
        let token = input.next_token();
        assert_eq!(token, Some(String::from(":")));
        let token = input.next_token();
        assert_eq!(token, Some(String::from("START")));
        let token = input.next_token();
        assert_eq!(token, Some(String::from("42")));
        let token = input.next_token();
        assert_eq!(token, Some(String::from("EMIT")));
        let token = input.next_token();
        assert_eq!(token, Some(String::from(";")));
        let token = input.next_token();
        assert!(token.is_none());
    }

}