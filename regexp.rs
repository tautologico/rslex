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
use parser::is_eof;

#[deriving(Eq)]
enum Token { LBrack, RBrack, Id(~str), LParen, RParen, Asterisk, 
             Plus, Bar, Dash, String(~str), Eof }

enum Ast { Symb(~str), Str(~str), Union(~Ast, ~Ast), 
           Conc(~Ast, ~Ast), Star(~Ast), OnePlus(~Ast), 
           CharClass(char, char), Epsilon }


fn parse_string(buffer: &mut LookaheadBuffer, delim: char) -> ~str {
    let mut res : ~str = ~"";
    let mut c = buffer.next_char();
    while c != delim {
        if (is_eof(c)) {
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

#[inline]
fn match_next_token(buffer: &mut LookaheadBuffer, t: ~Token) {
    let rt = get_next_token(buffer);
    if rt != t {
        fail!(fmt!("Unexpeced token: expected %?, got %?\n", t, rt));
    }
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
        '-' => ~Dash,
        '\'' => ~String(parse_string(buffer, '\'')),
        '"' => ~String(parse_string(buffer, '"')),
        c if std::char::is_alphabetic(c) => ~Id(parse_id(buffer, c)),
        c if is_eof(c) => ~Eof,
        c => fail!(fmt!("Unexpected character: %c\n", c))
    }
}

#[test]
fn test_get_next_token() {
    let mut b1 = std::io::with_str_reader("'return'", LookaheadBuffer::new);
    assert!(get_next_token(&mut b1) == ~String(~"return"));

    let mut b2 = std::io::with_str_reader("return", LookaheadBuffer::new);
    assert!(get_next_token(&mut b2) == ~Id(~"return"));

    let mut b3 = std::io::with_str_reader("(['a'-'z'])(['A'-'Z'])*", LookaheadBuffer::new);
    assert!(get_next_token(&mut b3) == ~LParen);
    assert!(get_next_token(&mut b3) == ~LBrack);
    assert!(get_next_token(&mut b3) == ~String(~"a"));
    assert!(get_next_token(&mut b3) == ~Dash);
    assert!(get_next_token(&mut b3) == ~String(~"z"));
    assert!(get_next_token(&mut b3) == ~RBrack);
    assert!(get_next_token(&mut b3) == ~RParen);
    assert!(get_next_token(&mut b3) == ~LParen);
    assert!(get_next_token(&mut b3) == ~LBrack);
    assert!(get_next_token(&mut b3) == ~String(~"A"));
    assert!(get_next_token(&mut b3) == ~Dash);
    assert!(get_next_token(&mut b3) == ~String(~"Z"));
    assert!(get_next_token(&mut b3) == ~RBrack);
    assert!(get_next_token(&mut b3) == ~RParen);
    assert!(get_next_token(&mut b3) == ~Asterisk);

    let mut b4 = std::io::with_str_reader("letter (letter | digit)*", LookaheadBuffer::new);
    assert!(get_next_token(&mut b4) == ~Id(~"letter"));
    assert!(get_next_token(&mut b4) == ~LParen);
    assert!(get_next_token(&mut b4) == ~Id(~"letter"));
    assert!(get_next_token(&mut b4) == ~Bar);
    assert!(get_next_token(&mut b4) == ~Id(~"digit"));
    assert!(get_next_token(&mut b4) == ~RParen);
    assert!(get_next_token(&mut b4) == ~Asterisk);
}

// regexp := union
// union  := union '|' concat | concat
// concat := concat factor | factor
// factor := (regexp) | regexp'*' | regexp'+' | class | id | str
// class  := '[' (char | range)* ']'
// range  := char'-'char

// parse a regexp from buffer until one of the terminators in term occurs
pub fn parse_regexp(buffer: &mut LookaheadBuffer, term: &[~str]) -> ~Ast {
    parse_union(buffer, term)
}

fn parse_union(buffer: &mut LookaheadBuffer, term: &[~str]) -> ~Ast {
    let left = parse_concat(buffer, term);
    if get_next_token(buffer) == ~Bar {
        let right = parse_union(buffer, term);
        ~Union(left, right)
    }
    else {
        left
    }
}

fn parse_concat(buffer: &mut LookaheadBuffer, term: &[~str]) -> ~Ast {
    let left = parse_factor(buffer, term);
    let right = parse_factor(buffer, term);
    ~Conc(left, right)
}

fn trailing_closure(buffer: &mut LookaheadBuffer) -> Option<char> {
    skip_whitespace(buffer);
    match buffer.next_char() {
        '*' => Some('*'),
        '+' => Some('+'),
        c => { buffer.return_char(c); None }
    }
}

fn parse_character_class(buffer: &mut LookaheadBuffer) -> ~Ast {
    ~Epsilon         // TODO
}

fn parse_factor(buffer: &mut LookaheadBuffer, term: &[~str]) -> ~Ast {
    let pre = match get_next_token(buffer) {
        ~LParen => { let e = parse_regexp(buffer, term); 
                     match_next_token(buffer, ~RParen); 
                     e }
        ~LBrack => parse_character_class(buffer),
        ~Id(s) => ~Symb(s),
        ~String(s) => ~Str(s),  // TODO: check for END
        _ => ~Epsilon        // TODO: error
    };
    match trailing_closure(buffer) {
        Some('*') => ~Star(pre),
        Some('+') => ~OnePlus(pre),
        Some(_) => fail!("Unexpected closure character"),
        None => pre
    }
}
