//
// rslex - a lexer generator for rust
//
// buffer.rs
// A lookahead buffer for input
//
// Andrei de A. Formiga, 2013-10-02
//

extern mod std;

use std::io::{SeekEnd, SeekSet};
use std::io::Seek;
use std::io::Reader;
//use std::io::Writer;
use std::io::File;
use std::str::CharIterator;

/// A lookahead buffer for reading input characters
struct Buffer2 {
    contents: ~str,
    index: uint            // TODO: change to iterator to avoid encoding issues
}

impl Buffer2 {
    fn new(s: ~str) -> Buffer2 {
        Buffer2 { contents: s, index: 0 }
    }

    fn from_file(fname: &str) -> Buffer2 {
        let cont = file_contents(fname);
        Buffer2 { contents: cont, index: 0 }
    }

    pub fn len(&self) -> uint {
        self.contents.len()
    }

    pub fn is_depleted(&self) -> bool {
        self.contents.len() == self.index
    }

    /// Returns the next char from the buffer. Behavior undefined 
    /// if the buffer is depleted.
    fn next_char(&mut self) -> char {
        let res = self.contents[self.index];
        if self.index < self.contents.len() {
            self.index = self.index + 1;
        }
        res as char     // FIX: assume ASCII encoding
    }

    fn return_char(&mut self) {
        if self.index > 0 {
            self.index = self.index - 1;
        }
    }
}

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
    fn new(s: &'r str) -> LookaheadBuffer<'r> {
        LookaheadBuffer { contents: s, iter: s.iter(), peek: None }
    }

    fn len(&self) -> uint {
        self.contents.len()
    }

    fn next_char(&mut self) -> Option<char> {
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
    fn return_char(&mut self, c: char) {
        self.peek = Some(c)
    }
}



// --- tests ----------------------------------------------------
#[cfg(test)]
mod buffer_tests {
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
        let mut iter = "abcdef".iter().peekable();
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
    }
}


// --- utility functions ----------------------------------------
fn file_contents(name: &str) -> ~str {
    let mut f = open_or_fail(name);
    read_contents(&mut f)
}

fn get_size(f: &mut File) -> u64 {
    f.seek(0, SeekEnd);
    let res = f.tell();
    f.seek(0, SeekSet);
    res
}

fn read_contents(f: &mut File) -> ~str {
    let size = get_size(f) as uint;
    let mut contents = std::vec::from_elem(size as uint, 0x00_u8);
    match f.read(contents) {
        Some(l) if l == size => std::str::from_utf8(contents),
        _ => fail!("Could not read file\n")
    }
}

fn open_or_fail(name: &str) -> File {
    match File::open(&Path::new(name)) {
        Some(f) => f,
        None => fail!("Could not open file\n")
    }
}
