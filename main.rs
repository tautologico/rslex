//
// rslex - a lexer generator for rust
//
// main.rs
// The main program 
//
// Andrei de A. Formiga, 2013-07-31
//

#[feature(globs)];

//mod parser;
mod buffer;
mod regexp;
mod io;

struct Definition {
    id: ~str,
    val: ~str
}

struct LexerSpec {
    defs: ~[~str],
    rules: ~[~str],
    code: ~[~str]
}

fn usage(progname: &str) {
    println!("Usage: {} <input file>", progname)
}

enum ReadState { Definitions, Rules, Code, }

// fn parse_definition(line: ~str) -> Definition {
// }

fn build_desc_from_lines(lines: ~[~str]) -> ~LexerSpec {
    let mut state = Definitions;
    let mut ds : ~[~str] = ~[];
    let mut rs : ~[~str] = ~[];
    let mut cd : ~[~str] = ~[];

    for line in lines.iter() {
        if line.starts_with("%%") {
            state = match state { 
                Definitions => Rules,
                Rules => Code,
                Code => Code // should be an error
            }
        }
        else {
            match state {
                Definitions => ds = std::vec::append_one(ds, line.to_owned()),
                Rules => rs = std::vec::append_one(rs, line.to_owned()),
                Code => cd = std::vec::append_one(cd, line.to_owned())
            }
        }
    }
    ~LexerSpec { defs: ds, rules: rs, code: cd }
}

// fn read_lexer_spec(specfile: &str) -> Result<~LexerSpec, ~str> {
//     match std::io::file_reader(&Path(specfile)) {
//         Ok(reader) => { parser::parse_lexer_spec(reader); Ok(build_desc_from_lines(reader.read_lines())) }
//         Err(msg) => Err(~"Error opening file: " + msg)
//     }
// }

fn print_spec(spec: ~LexerSpec) {
    println("%% Definitions:");
    for def in spec.defs.iter() {  println(*def);  }
    println("%% Rules:");
    for rule in spec.rules.iter() {  println(*rule);  }
    println("%% Code:");
    for cd in spec.code.iter() {  println(*cd);  }
}

fn main() {
    let args = std::os::args();

    if args.len() <= 1 {
        usage(args[0]);
    }
    else { 
        println!("Ok, we'll now process your lex file {}", args[1]); 
        // match read_lexer_spec(args[1]) {
        //     Ok(desc) => print_spec(desc),
        //     Err(msg) => println(msg)
        // }
    }
}
