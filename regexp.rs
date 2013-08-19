//
// rslex - a lexer generator for rust
//
// regexp.rs
// Regexp parsing and representation
//
// Andrei de A. Formiga, 2013-08-09
//

extern mod std;

use parser::LookaheadBuffer;
use parser::skip_whitespace;
use parser::EOF;

enum Token { LBrack, RBrack, Id(~str), LParen, RParen, Asterisk, 
             Plus, Bar, String(~str), Eof }

enum Ast { Symb(~str), Union(~Ast, ~Ast), 
           Conc(~Ast, ~Ast), Star(~Ast), Epsilon }


fn parse_string(buffer: &mut LookaheadBuffer, delim: char) -> ~str {
    let mut res : ~str = ~"";
    let mut c = buffer.next_char();
    while c != delim {
        if (c == EOF) {
            fail!(fmt!("Unexpected end of file. Expected closing %c", delim));
        }
        res.push_char(c);
        c = buffer.next_char();
    }
    res
}

#[test]
fn test_parse_string() {
    let mut b1 = std::io::with_str_reader("abc'* ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_string(&mut b1, '\''), &~"abc"));
    assert!(b1.next_char() == '*');

    let mut b2 = std::io::with_str_reader("abc'def\"  ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_string(&mut b2, '"'), &~"abc'def"));
    assert!(b2.next_char() == ' '); 
}

#[test]
#[should_fail]
fn unclosed_string() {
    let mut b1 = std::io::with_str_reader("abc'def  ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_string(&mut b1, '"'), &~"abc'def"));
}

#[inline]
fn is_id_char(c: char) -> bool {
    std::char::is_alphanumeric(c) || c == '_'
}

fn parse_id(buffer: &mut LookaheadBuffer, first: char) -> ~str {
    let mut res : ~str = ~"";
    res.push_char(first);
    let mut c = buffer.next_char();
    while is_id_char(c) {
        res.push_char(c);
        c = buffer.next_char();
    }
    buffer.return_char(c);
    res
}

#[test]
fn test_parse_id() {
    let mut b1 = std::io::with_str_reader("abc'* ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_id(&mut b1, 'x'), &~"xabc"));
    assert!(b1.next_char() == '\'');

    let mut b2 = std::io::with_str_reader("bc_def   ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_id(&mut b2, 'a'), &~"abc_def"));

    let mut b3 = std::io::with_str_reader("_times|'xy')*   ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_id(&mut b3, 'n'), &~"n_times"));
    assert!(b3.next_char() == '|');

    let mut b4 = std::io::with_str_reader(" +xy)*   ", LookaheadBuffer::new);
    assert!(std::str::eq(&parse_id(&mut b4, 'n'), &~"n"));
    assert!(b4.next_char() == ' ');
    assert!(b4.next_char() == '+');
    assert!(b4.next_char() == 'x');
    assert!(std::str::eq(&parse_id(&mut b4, 'x'), &~"xy"));
}

fn get_next_token(buffer: &mut LookaheadBuffer) -> ~Token {
    skip_whitespace(buffer);
    match buffer.next_char() {
        '[' => ~LBrack,
        ']' => ~RBrack,
        '(' => ~LParen,
        ')' => ~RParen,
        '*' => ~Asterisk,
        '+' => ~Plus,
        '|' => ~Bar,
        '\'' => ~String(parse_string(buffer, '\'')),
        '"' => ~String(parse_string(buffer, '"')),
        c if std::char::is_alphabetic(c) => ~Id(parse_id(buffer, c)),
        EOF => ~Eof,
        _ => ~Eof // TODO: signal error?
    }
}

fn parse_regexp(buffer: &mut LookaheadBuffer) {
    skip_whitespace(buffer);
    let c = buffer.next_char();
}

