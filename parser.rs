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
             DefsId, RulesId, CodeId, Id(~str), RegExp, Eof }

enum Block { Rules, Defs, Code }

// fn match_next_token() -> bool {
// }

fn get_next_token(buffer: &mut LookaheadBuffer) -> ~Token {
    let mut c = buffer.next_char();
    let mut done = false;
    let mut res = ~Eof;
    while !done {
        skip_whitespace(buffer);
        if is_eof(c) {
            done = true;
        }
        else {
            match c {
                '{' => { res = ~LBrace; done = true; },
                _   => { done = true; }   // FIX
            }
        }
    }
    res
}

fn parse_id(buffer: &mut LookaheadBuffer, first: char) -> ~str {
    let mut res : ~str = ~"";
    let mut c = buffer.next_char();
    res.push_char(first);
    while !is_eof(c) && !std::char::is_whitespace(c) {
        res.push_char(c);
        c = buffer.next_char();
    }
    buffer.return_char(c);
    res    
}

#[test]
fn test_parse_id() {
    let mut buffer = std::io::with_str_reader("hombas   ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_id(&mut buffer, 's'), &~"shombas"));

    let mut buffer2 = std::io::with_str_reader(" or die ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_id(&mut buffer2, 'b'), &~"b"));
}

fn parse_toplevel_block(buffer: &mut LookaheadBuffer) {
}

// fn parse_regexp() {
// }

pub fn parse_lexer_spec(reader: @Reader) {
    let mut buffer = LookaheadBuffer::new(reader);
    parse_toplevel_block(&mut buffer);
}

