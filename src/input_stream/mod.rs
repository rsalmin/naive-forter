use std::collections::VecDeque;
use std::fmt;

/// Input Stream for Forth

#[derive(Clone)]
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

    /// check is first token equal to give one,
    /// return true if first token is exist and it's equal to given one, otherwise returns false

    pub fn is_first_token_equal(&self, word : &str) -> bool {
       match self.tokens.get(0).as_ref() {
           Some(&w) => w == word,
           None => false,
       }
    }

    /// append one InputStream to another

    pub fn append(&mut self, other : InputStream)  {
        let mut o = other;
        self.tokens.append(&mut o.tokens);
    }

    /// preppend given InputStream

    pub fn prepend(&mut self, mut other : InputStream)  {
        other.tokens.append(&mut self.tokens);
        self.tokens = other.tokens;
    }
    /// clear content of InputStream

    pub fn clear(&mut self)  {
        self.tokens.clear();
    }

    /// check is given token is exists in the list of tokens
    /// return true if the token exists, false otherwise

    pub fn is_token_exists(&self, word : &str) -> bool {
       self.tokens.iter().find(|&x| x == word).is_some()
    }

    /// returns some string from current stream position till the first occurence of `word`
    /// `word` must be surrounded by spaces
    /// if end word (utf8) can't be found returns None
    /// `word` not returned and consumed

    pub fn take_until_first(&mut self, word : &str) -> Option<InputStream> {
         if let Some( n ) = self.tokens.iter().position(|x| x == word) {
            let mut tokens = self.tokens.split_off(n); // keeps n elements, returns last
            std::mem::swap(&mut self.tokens, &mut tokens);
            self.tokens.pop_front(); // consume input word
            let r = InputStream { tokens };
            return Some(r);
        }

        None
    }

    /// returns some string from current stream position till the last occurence of `word`
    /// `word` must be surrounded by spaces
    /// if end word (utf8) can't be found returns None
    /// `word` not returned and consumed

    pub fn take_until_last(&mut self, word : &str) -> Option<InputStream> {
         if let Some( n ) = self.tokens.iter().rev().position(|x| x == word) {
            let mut tokens = self.tokens.split_off(self.tokens.len() - 1 - n); // keeps n elements, returns last
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
    fn take_until_first() {
        let mut input = InputStream::from(" : START 42 EMIT ; ");
        let part = input.take_until_first(";");
        assert_eq!(part, Some(InputStream::from(" : START 42 EMIT ")));
        let part = input.take_until_first(";");
        assert!(part.is_none());

        let mut input = InputStream::from(" 42 THEN");
        let part = input.take_until_first("THEN");
        assert_eq!(part, Some(InputStream::from(" 42 ")));

        let part = input.take_until_first("THEN");
        assert!(part.is_none());
    }

    #[test]
    fn take_until_last() {
        let mut input = InputStream::from(" : START 42 EMIT ; ");
        let part = input.take_until_last(";");
        assert_eq!(part, Some(InputStream::from(" : START 42 EMIT ")));
        let part = input.take_until_last(";");
        assert!(part.is_none());

        let mut input = InputStream::from(" 42 THEN THEN THEN");
        let part = input.take_until_last("THEN");
        assert_eq!(part, Some(InputStream::from(" 42 THEN THEN")));

        let part = input.take_until_last("THEN");
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

    #[test]
    fn is_first_token_equal() {
        let input = InputStream::from("     : START 42 EMIT ;");
        assert!( input.is_first_token_equal(":") );

        let input = InputStream::from("START 42 EMIT ;");
        assert!( ! input.is_first_token_equal(":") );

        let input = InputStream::from("START 42 EMIT ;");
        assert!( input.is_first_token_equal("START") );

        let input = InputStream::from("");
        assert!( ! input.is_first_token_equal("START") );
     }

    #[test]
    fn is_token_exists() {
        let input = InputStream::from("     : START 42 EMIT ;");
        assert!( input.is_token_exists("EMIT") );
        let input = InputStream::from("     : START 42 EMIT ;");
        assert!( input.is_token_exists(";") );
        let input = InputStream::from("     : START 42 EMIT ;");
        assert!( input.is_token_exists(":") );

        let input = InputStream::from("     : START 42 EMIT ;");
        assert!( ! input.is_token_exists("HI") );
   }

    #[test]
    fn append_prepend_and_clear() {
        let mut input1 = InputStream::from("     : START 42 EMIT ");
        let input2 = InputStream::from("42 EMIT ;");
        input1.append(input2);
        assert_eq!(input1, InputStream::from(" : START 42 EMIT 42 EMIT ;"));

        input1.clear();
        assert_eq!(input1, InputStream::from(""));

        let mut input1 = InputStream::from("     : START 42 EMIT ");
        let input2 = InputStream::from("42 EMIT ;");
        input1.prepend(input2);
        assert_eq!(input1, InputStream::from("42 EMIT ; : START 42 EMIT "));
   }
}
