//
// rslex - a lexer generator for rust
//
// buffer.rs
// A lookahead buffer for input
//
// Andrei de A. Formiga, 2013-10-02
//

extern crate std;

use std::str::Chars;

/// A position in the input
struct Pos {
    /// line number (starting at 1) 
    line: uint,
    /// column number (starting at 0)
    col: uint
}

impl Pos {
    pub fn start() -> Pos {
        Pos { line: 1, col: 0 }
    }

    pub fn next(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.col = 0;
        }
        else {
            self.col += 1;
        }
    }
}

type CharPos = uint;

/// Allows the use of one "lookahead" character, which can be 
/// returned to the stream. Basically similar to a `Peekable`
/// iterator but with a more convenient interface 
/// (for a case where peek is used all the time 
///  and it almost always should advance the iterator)
struct LookaheadBuffer<'r> {
    contents: &'r str,
    iter: Chars<'r>,
    peek: Option<char>,
    pos: CharPos
}

impl<'r> LookaheadBuffer<'r> {
    pub fn new(s: &'r str) -> LookaheadBuffer<'r> {
        LookaheadBuffer { contents: s, iter: s.chars(), peek: None, pos: 0 }
    }

    pub fn len(&self) -> uint {
        self.contents.len()
    }

    fn get_next_char(&mut self) -> Option<char> {
        match self.peek {
            None => self.iter.next(),
            Some(c) => { 
                self.peek = None;
                Some(c)
            }
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        let res = self.get_next_char();
        // update position
        match res {
            Some(_) => self.pos += 1,
            None => ()
        }
        res
    }

    /// Returns `c` to the character stream, to be returned as the 
    /// next char. If `return_char` is called twice without an intervening
    /// `next_char`, the buffer will forget the previous returned character. 
    pub fn return_char(&mut self, c: char) {
        self.pos -= 1;
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

    #[test]
    fn position() {
        let mut buffer = LookaheadBuffer::new("   abc   ");
        assert_eq!(buffer.pos, 0);
        buffer.skip_whitespace();
        assert_eq!(buffer.pos, 3);
        assert_eq!(buffer.next_char(), Some('a'));
        assert_eq!(buffer.pos, 4);
        assert_eq!(buffer.next_char(), Some('b'));
        assert_eq!(buffer.pos, 5);
        assert_eq!(buffer.next_char(), Some('c'));
        assert_eq!(buffer.pos, 6);
        buffer.return_char('c');
        assert_eq!(buffer.pos, 5);
        assert_eq!(buffer.next_char(), Some('c'));
        assert_eq!(buffer.pos, 6);
        buffer.skip_whitespace();
        assert_eq!(buffer.pos, 9);
    }
}

