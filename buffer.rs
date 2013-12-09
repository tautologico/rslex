//
// rslex - a lexer generator for rust
//
// buffer.rs
// A lookahead buffer for input
//
// Andrei de A. Formiga, 2013-10-02
//

extern mod std;

use std::str::CharIterator;

/// Allows the use of one "lookahead" character, which can be 
/// returned to the stream. Basically similar to a `Peekable`
/// iterator but with a more convenient interface 
/// (for a case where peek is used all the time 
///  and it almost always should advance the iterator)
struct LookaheadBuffer<'r> {
    contents: &'r str,
    iter: CharIterator<'r>,
    peek: Option<char>
}

impl<'r> LookaheadBuffer<'r> {
    pub fn new(s: &'r str) -> LookaheadBuffer<'r> {
        LookaheadBuffer { contents: s, iter: s.chars(), peek: None }
    }

    pub fn len(&self) -> uint {
        self.contents.len()
    }

    pub fn next_char(&mut self) -> Option<char> {
        match self.peek {
            None => self.iter.next(),
            Some(c) => { 
                self.peek = None;
                Some(c)
            }
        }
    }

    /// Returns `c` to the character stream, to be returned as the 
    /// next char. If `return_char` is called twice without an intervening
    /// `next_char`, the buffer will forget the previous returned character. 
    pub fn return_char(&mut self, c: char) {
        self.peek = Some(c)
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            match self.next_char() {
                None => break,
                Some(c) if !std::char::is_whitespace(c) => { self.return_char(c); break }
                Some(_) => ()
            }
        }
    }
}



// --- tests ----------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_buffer_use() {
        let mut buffer = LookaheadBuffer::new("abcdef");
        assert_eq!(buffer.len(), 6);
        assert_eq!(buffer.next_char(), Some('a'));
        assert_eq!(buffer.next_char(), Some('b'));
        assert_eq!(buffer.next_char(), Some('c'));
        assert_eq!(buffer.next_char(), Some('d'));
        assert_eq!(buffer.next_char(), Some('e'));
        assert_eq!(buffer.next_char(), Some('f'));
        assert_eq!(buffer.next_char(), None);
    }

    #[test]
    fn no_buffer_use_iterator() {
        let mut iter = "abcdef".chars().peekable();
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('b'));
        
        match iter.peek() {
            None => fail!("Should not be none"),
            Some(c) => assert_eq!(*c, 'c')
        }

        assert_eq!(iter.next(), Some('c'));
    }

    #[test]
    fn buffer_use_return_variable() {
        let mut buffer = LookaheadBuffer::new("abcdef");
        assert_eq!(buffer.next_char(), Some('a'));
        let c = match buffer.next_char() {
            None => fail!("Should not be none"),
            Some(ch) => ch
        };

        assert_eq!(c, 'b');

        buffer.return_char(c);

        assert_eq!(buffer.next_char(), Some('b'));
    }

    #[test]
    fn buffer_use() {
        let mut buffer = LookaheadBuffer::new("abcdef");
        assert_eq!(buffer.next_char(), Some('a'));
        buffer.return_char('a');
        assert_eq!(buffer.next_char(), Some('a'));
        assert_eq!(buffer.next_char(), Some('b'));
        assert_eq!(buffer.next_char(), Some('c'));
        buffer.return_char('c');
        assert_eq!(buffer.next_char(), Some('c'));
        assert_eq!(buffer.next_char(), Some('d'));
        assert_eq!(buffer.next_char(), Some('e'));
        assert_eq!(buffer.next_char(), Some('f'));
        assert_eq!(buffer.next_char(), None);
        buffer.return_char('f');
        assert_eq!(buffer.next_char(), Some('f'));
        assert_eq!(buffer.next_char(), None);
        assert_eq!(buffer.next_char(), None);
    }

    #[test]
    fn test_skip_ws() {
        let mut buffer = LookaheadBuffer::new("   abc   ");
        buffer.skip_whitespace();
        assert_eq!(buffer.next_char(), Some('a'));
        buffer.skip_whitespace();
        assert_eq!(buffer.next_char(), Some('b'));
        buffer.skip_whitespace();
        assert_eq!(buffer.next_char(), Some('c'));
        buffer.skip_whitespace();
        assert_eq!(buffer.next_char(), None);
    }
}

