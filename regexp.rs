//
// rslex - a lexer generator for rust
//
// regexp.rs
// Regexp parsing and representation
//
// Andrei de A. Formiga, 2013-08-09
//

extern mod std;

use buffer::LookaheadBuffer;
//use parser::skip_whitespace;

#[deriving(Eq)]
enum Token { LBrack, RBrack, Id(~str), LParen, RParen, Asterisk, 
             Plus, Bar, Dash, String(~str), Eof }

enum Ast { Symb(~str), Str(~str), Union(~Ast, ~Ast),
           Conc(~Ast, ~Ast), Star(~Ast), OnePlus(~Ast), 
           CharClass(char, char), Epsilon }

fn parse_string(buffer: &mut LookaheadBuffer, delim: char) -> ~str {
    let mut res : ~str = ~"";
    loop {
        match buffer.next_char() {
            None => fail!("Unexpected end of file. Expected closing {}", delim),
            Some(c) if c == delim => break,
            Some(c) => res.push_char(c)
        }
    }
    res
}

#[test]
fn test_parse_string() {
    let mut b1 = LookaheadBuffer::new("abc'* ");
    assert_eq!(parse_string(&mut b1, '\''), ~"abc");
    assert_eq!(b1.next_char(), Some('*'));

    let mut b2 = LookaheadBuffer::new("abc'def\"  ");
    assert_eq!(parse_string(&mut b2, '"'), ~"abc'def");
    assert_eq!(b2.next_char(), Some(' ')); 
}

#[test]
#[should_fail]
fn unclosed_string() {
    let mut b1 = LookaheadBuffer::new("abc'def  ");
    assert_eq!(parse_string(&mut b1, '"'), ~"abc'def");
}

#[inline]
fn is_id_char(c: char) -> bool {
    std::char::is_alphanumeric(c) || c == '_'
}

fn parse_id(buffer: &mut LookaheadBuffer, first: char) -> ~str {
    let mut res : ~str = ~"";
    res.push_char(first);
    loop {
        match buffer.next_char() {
            Some(c) if is_id_char(c) => res.push_char(c),
            Some(c) => { buffer.return_char(c); break }
            None => break
        }
    }
    res
}

#[test]
fn test_parse_id() {
    let mut b1 = LookaheadBuffer::new("abc'* ");
    assert_eq!(parse_id(&mut b1, 'x'), ~"xabc");
    assert_eq!(b1.next_char(), Some('\''));

    let mut b2 = LookaheadBuffer::new("bc_def   ");
    assert_eq!(parse_id(&mut b2, 'a'), ~"abc_def");

    let mut b3 = LookaheadBuffer::new("_times|'xy')*   ");
    assert_eq!(parse_id(&mut b3, 'n'), ~"n_times");
    assert_eq!(b3.next_char(), Some('|'));

    let mut b4 = LookaheadBuffer::new(" +xy)*   ");
    assert_eq!(parse_id(&mut b4, 'n'), ~"n");
    assert_eq!(b4.next_char(), Some(' '));
    assert_eq!(b4.next_char(), Some('+'));
    assert_eq!(b4.next_char(), Some('x'));
    assert_eq!(parse_id(&mut b4, 'x'), ~"xy");
}

#[inline]
fn match_next_token(buffer: &mut LookaheadBuffer, t: Token) {
    let rt = get_next_token(buffer);
    if rt != t {
        fail!("Unexpeced token: expected {:?}, got {:?}", t, rt);
    }
}

fn get_next_token(buffer: &mut LookaheadBuffer) -> Token {
    buffer.skip_whitespace();
    match buffer.next_char() {
        Some('[') => LBrack,
        Some(']') => RBrack,
        Some('(') => LParen,
        Some(')') => RParen,
        Some('*') => Asterisk,
        Some('+') => Plus,
        Some('|') => Bar,
        Some('-') => Dash,
        Some('\'') => String(parse_string(buffer, '\'')),
        Some('"') => String(parse_string(buffer, '"')),
        Some(c) if std::char::is_alphabetic(c) => Id(parse_id(buffer, c)),
        None => Eof,
        Some(c) => fail!("Unexpected character: {}", c)
    }
}

#[test]
fn test_get_next_token() {
    let mut b1 = LookaheadBuffer::new("'return'");
    assert_eq!(get_next_token(&mut b1), String(~"return"));

    let mut b2 = LookaheadBuffer::new("return");
    assert_eq!(get_next_token(&mut b2), Id(~"return"));

    let mut b3 = LookaheadBuffer::new("(['a'-'z'])(['A'-'Z'])*");
    assert_eq!(get_next_token(&mut b3), LParen);
    assert_eq!(get_next_token(&mut b3), LBrack);
    assert_eq!(get_next_token(&mut b3), String(~"a"));
    assert_eq!(get_next_token(&mut b3), Dash);
    assert_eq!(get_next_token(&mut b3), String(~"z"));
    assert_eq!(get_next_token(&mut b3), RBrack);
    assert_eq!(get_next_token(&mut b3), RParen);
    assert_eq!(get_next_token(&mut b3), LParen);
    assert_eq!(get_next_token(&mut b3), LBrack);
    assert_eq!(get_next_token(&mut b3), String(~"A"));
    assert_eq!(get_next_token(&mut b3), Dash);
    assert_eq!(get_next_token(&mut b3), String(~"Z"));
    assert_eq!(get_next_token(&mut b3), RBrack);
    assert_eq!(get_next_token(&mut b3), RParen);
    assert_eq!(get_next_token(&mut b3), Asterisk);

    let mut b4 = LookaheadBuffer::new("letter (letter | digit)*");
    assert_eq!(get_next_token(&mut b4), Id(~"letter"));
    assert_eq!(get_next_token(&mut b4), LParen);
    assert_eq!(get_next_token(&mut b4), Id(~"letter"));
    assert_eq!(get_next_token(&mut b4), Bar);
    assert_eq!(get_next_token(&mut b4), Id(~"digit"));
    assert_eq!(get_next_token(&mut b4), RParen);
    assert_eq!(get_next_token(&mut b4), Asterisk);
}

// regexp := union
// union  := union '|' concat | concat
// concat := concat factor | factor
// factor := (regexp) | regexp'*' | regexp'+' | class | id | str
// class  := '[' (char | range)* ']'
// range  := char'-'char

// parse a regexp from buffer until one of the terminators in term occurs
pub fn parse_regexp(buffer: &mut LookaheadBuffer, term: &[~str]) -> Ast {
    parse_union(buffer, term)
}

fn parse_union(buffer: &mut LookaheadBuffer, term: &[~str]) -> Ast {
    let left = parse_concat(buffer, term);
    if get_next_token(buffer) == Bar {
        let right = parse_union(buffer, term);
        Union(~left, ~right)
    }
    else {
        left
    }
}

fn parse_concat(buffer: &mut LookaheadBuffer, term: &[~str]) -> Ast {
    let left = parse_factor(buffer, term);
    let right = parse_factor(buffer, term);
    Conc(~left, ~right)
}

fn trailing_closure(buffer: &mut LookaheadBuffer) -> Option<char> {
    buffer.skip_whitespace();
    match buffer.next_char() {
        Some('*') => Some('*'),
        Some('+') => Some('+'),
        Some(c) => { buffer.return_char(c); None },
        None => None
    }
}

fn parse_character_class(buffer: &mut LookaheadBuffer) -> Ast {
    Epsilon         // TODO
}

fn parse_factor(buffer: &mut LookaheadBuffer, term: &[~str]) -> Ast {
    let pre = match get_next_token(buffer) {
        LParen => { let e = parse_regexp(buffer, term); 
                    match_next_token(buffer, RParen); 
                    e }
        LBrack => parse_character_class(buffer),
        Id(s) => Symb(s),
        String(s) => Str(s),  // TODO: check for END
        _ => Epsilon          // TODO: error
    };
    match trailing_closure(buffer) {
        Some('*') => Star(~pre),
        Some('+') => OnePlus(~pre),
        Some(_) => fail!("Unexpected closure character"),
        None => pre
    }
}
