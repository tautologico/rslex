//
// rslex - a lexer generator for rust
//
// regexp.rs
// Regexp parsing and representation
//
// Andrei de A. Formiga, 2013-08-09
//

use parser::LookaheadBuffer;
use parser::skip_whitespace;

enum ReToken { LBrack, RBrack, ReId(~str), LParen, RParen, Asterisk, 
               Plus, Bar, String(~str) }

enum ReAst { Symb(~str), Union(~ReAst, ~ReAst), 
             Conc(~ReAst, ~ReAst), Star(~ReAst), Epsilon }


fn parse_regexp(buffer: &mut LookaheadBuffer) {
    skip_whitespace(buffer);
    let c = buffer.next_char();
}

