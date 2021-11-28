use std::str::Chars;

/// Input Stream for Forth
pub struct InputStream<'a> {
    iter : Chars<'a>,
}

impl<'a> InputStream<'a> {
    pub fn from<'b>(s : &'b str) -> InputStream<'b> {
        InputStream { iter : s.chars()  }
    }

    /// returns some string from current stream position till the occurence of `end`
    /// if end char (utf8) can't be found returns None
    /// `end` char not returned and consumed
    pub fn take_until(&mut self, end : char) -> Option<String> {
        if self.iter.clone().find(|&x| x == end).is_none() {
            return None;
        }
        Some( self.iter.by_ref().take_while(|&x| x != end).collect::<String>() )
    }

    /// returns next Forth token if present, all leading whitespaces(utf8) are consumed
    /// if none token can be found, returns None
    /// consumes one whitespaces (if any) after the each token - who cares?
    pub fn next_token(&mut self) -> Option<String> {
        let trimed = self.iter.by_ref().skip_while(|x| x.is_whitespace() );   // Trim the beginig
        let s = trimed.take_while(|&x| ! x.is_whitespace()).collect::<String>(); //get token
        if s.is_empty() {
            return None;
        }
        Some( s )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_until() {
        let mut input = InputStream::from(" : START 42 EMIT ; ");
        let part = input.take_until(';');
        assert_eq!(part, Some(String::from(" : START 42 EMIT ")));
        let part = input.take_until(';');
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