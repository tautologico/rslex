//
// rslex - a lexer generator for rust
//
// buffer.rs
// A lookahead buffer for input
//
// Andrei de A. Formiga, 2013-10-02
//

extern mod std;

use std::rt::io::file::open;
use std::rt::io::{Open, Create, Read, Write, SeekEnd, SeekSet};
use std::rt::io::Seek;
use std::rt::io::Reader;
use std::rt::io::Writer;
use std::rt::io::FileStream;

/// A lookahead buffer for reading input characters
struct LookaheadBuffer {
    contents: ~str,
    index: uint           // TODO: change to iterator to avoid encoding issues
}

impl LookaheadBuffer {
    fn new(s: ~str) -> LookaheadBuffer {
        LookaheadBuffer { contents: s, index: 0 }
    }

    fn from_file(fname: &str) -> LookaheadBuffer {
        let cont = file_contents(fname);
        LookaheadBuffer { contents: cont, index: 0 }
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

// --- tests ----------------------------------------------------
#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn no_buffer_use() {
        let mut buffer = LookaheadBuffer::new(~"abcdef");
        assert_eq!(buffer.len(), 6);
        assert!(!buffer.is_depleted());
        assert_eq!(buffer.next_char(), 'a');
        assert_eq!(buffer.next_char(), 'b');
        assert_eq!(buffer.next_char(), 'c');
        assert_eq!(buffer.next_char(), 'd');
        assert_eq!(buffer.next_char(), 'e');
        assert_eq!(buffer.next_char(), 'f');
        assert!(buffer.is_depleted());
    }

    #[test]
    fn buffer_use() {
        let mut buffer = LookaheadBuffer::new(~"abcdef");
        assert_eq!(buffer.next_char(), 'a');
        buffer.return_char();
        assert_eq!(buffer.next_char(), 'a');
        assert_eq!(buffer.next_char(), 'b');
        assert_eq!(buffer.next_char(), 'c');
        buffer.return_char();
        assert_eq!(buffer.next_char(), 'c');
        assert_eq!(buffer.next_char(), 'd');
        assert_eq!(buffer.next_char(), 'e');
        assert_eq!(buffer.next_char(), 'f');
        assert!(buffer.is_depleted());
        buffer.return_char();
        assert!(!buffer.is_depleted());
        assert_eq!(buffer.next_char(), 'f');
        assert!(buffer.is_depleted());
    }

    #[test]
    fn from_file() {
        let mut buffer = LookaheadBuffer::from_file("Makefile");
        assert_eq!(buffer.next_char(), 'a');
        assert_eq!(buffer.next_char(), 'l');
        assert_eq!(buffer.next_char(), 'l');
        assert_eq!(buffer.next_char(), ':');
        assert_eq!(buffer.next_char(), ' ');
        assert_eq!(buffer.next_char(), 'r');
        assert_eq!(buffer.next_char(), 's');
        assert_eq!(buffer.next_char(), 'l');
        assert_eq!(buffer.next_char(), 'e');
        assert_eq!(buffer.next_char(), 'x');
        assert!(!buffer.is_depleted());        
    }
}


// --- utility functions ----------------------------------------
fn file_contents(name: &str) -> ~str {
    let mut f = open_or_fail(name);
    read_contents(&mut f)
}

fn get_size(f: &mut FileStream) -> u64 {
    f.seek(0, SeekEnd);
    let res = f.tell();
    f.seek(0, SeekSet);
    res
}

fn read_contents(f: &mut FileStream) -> ~str {
    let size = get_size(f) as uint;
    let mut contents = std::vec::from_elem(size as uint, 0x00_u8);
    match f.read(contents) {
        Some(l) if l == size => std::str::from_utf8(contents),
        _ => fail!("Could not read file\n")
    }
}

fn open_or_fail(name: &str) -> FileStream {
    match open(&Path(name), Open, Read) {
        Some(f) => f,
        None => fail!("Could not open file\n")
    }
}
