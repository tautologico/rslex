//
// rslex - a lexer generator for rust
//
// parser.rs
// Input file parsing
//
// Andrei de A. Formiga, 2013-08-04
//

extern mod std;

// --- utility functions -------------------------------------------

#[inline]
fn is_eof(c: char) -> bool {
    c == (-1 as char)
}


// --- the lookahead buffer ----------------------------------------
struct LookaheadBuffer {
    c: char,
    valid: bool,
    reader: @Reader
}

impl LookaheadBuffer {
    fn new(r: @Reader) -> LookaheadBuffer {
        LookaheadBuffer { c: ' ', valid: false, reader: r}
    }

    // FIX: should be read_char but doesn't work due to method resolution bug in rustc
    fn next_char(&mut self) -> char {
        if self.valid { 
            self.valid = false;
            self.c
        }
        else {
            self.reader.read_char()
        }
    }

    fn return_char(&mut self, c: char) {
        // assumes at most one char of lookahead
        assert!(self.valid == false);
        self.c = c;
        self.valid = true;
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;
    use std::io::with_str_reader;

    #[test]
    fn no_buffer_use() {
        let mut buffer = with_str_reader("abcdef", LookaheadBuffer::new);
        assert!(!buffer.valid);
        assert!(buffer.next_char() == 'a');
        assert!(buffer.next_char() == 'b');
        assert!(buffer.next_char() == 'c');
        assert!(!buffer.valid);
    }

    #[test]
    fn buffer_use() {
        let mut buffer = with_str_reader("abcdef", LookaheadBuffer::new);
        assert!(!buffer.valid);
        assert!(buffer.next_char() == 'a');
        buffer.return_char('a');
        assert!(buffer.valid);
        assert!(buffer.next_char() == 'a');
        assert!(!buffer.valid);
        assert!(buffer.next_char() == 'b');
        assert!(buffer.next_char() == 'c');
    }

    #[test]
    fn return_other_char() {
        let mut buffer = with_str_reader("abcdef", LookaheadBuffer::new);
        assert!(!buffer.valid);
        assert!(buffer.next_char() == 'a');
        buffer.return_char('x');
        assert!(buffer.valid);
        assert!(buffer.next_char() == 'x');
        assert!(!buffer.valid);
        assert!(buffer.next_char() == 'b');
        assert!(buffer.next_char() == 'c');
        assert!(!buffer.valid);
        buffer.return_char('c');
        assert!(buffer.valid);
        assert!(buffer.next_char() == 'c');
        assert!(!buffer.valid);
        assert!(buffer.next_char() == 'd');
        assert!(buffer.next_char() == 'e');
    }

    #[test]
    #[should_fail]
    fn return_other_char_expect_same() {
        let mut buffer = with_str_reader("abcdef", LookaheadBuffer::new);
        assert!(!buffer.valid);
        assert!(buffer.next_char() == 'a');
        buffer.return_char('x');
        assert!(buffer.valid);
        assert!(buffer.next_char() == 'a');
    }

    #[test]
    fn read_until_eof() {
        let mut buffer = with_str_reader("abc", LookaheadBuffer::new);
        assert!(buffer.next_char() == 'a');
        assert!(buffer.next_char() == 'b');
        assert!(buffer.next_char() == 'c');
        assert!(buffer.next_char() == -1 as char);
    }

    #[test]
    fn return_char_eof() {
        let mut buffer = with_str_reader("abc", LookaheadBuffer::new);
        assert!(buffer.next_char() == 'a');
        assert!(buffer.next_char() == 'b');
        assert!(buffer.next_char() == 'c');
        assert!(buffer.next_char() == -1 as char);        
        buffer.return_char(-1 as char);
        assert!(buffer.valid);
        assert!(buffer.next_char() == -1 as char);
    }
}


fn skip_whitespace(buffer: &mut LookaheadBuffer) {
    let mut c = buffer.next_char();
    while (!is_eof(c) && std::char::is_whitespace(c)) {
        c = buffer.next_char();
    }
    buffer.return_char(c);
}

#[test]
fn test_skip_ws() {
    let mut buffer = std::io::with_str_reader("   abc   ", LookaheadBuffer::new);
    skip_whitespace(&mut buffer);
    assert!(buffer.next_char() == 'a');
    skip_whitespace(&mut buffer);
    assert!(buffer.next_char() == 'b');
    skip_whitespace(&mut buffer);
    assert!(buffer.next_char() == 'c');
    skip_whitespace(&mut buffer);
    assert!(is_eof(buffer.next_char()));
}

// token types 
enum Token { LBrace, RBrace, Equals, Comma, Semicolon, 
             LParen, RParen, Star, Plus, Bar, 
             Defs, Rules, Code, Id(~str), RegExp }

// fn match_next_token() -> bool {
// }

// fn get_next_token(reader: @Reader) -> ~Token {
// }

// fn parse_regexp() {
// }

pub fn parse_lexer_spec(reader: @Reader) {
    let c : char = reader.read_char();
    println("Parsing")
}

