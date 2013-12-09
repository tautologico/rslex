//
// rslex - a lexer generator for rust
//
// io.rs
// I/O utility functions
//
// Andrei de A. Formiga, 2013-11-19
//

extern mod std;

use std::io::{SeekEnd, SeekSet};
use std::io::Seek;
use std::io::Reader;
use std::io::File;

pub fn file_contents(name: &str) -> ~str {
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
    //let size = get_size(f) as uint;
    //let mut contents = std::vec::from_elem(size as uint, 0x00_u8);
    // match f.read(contents) {
    //     Some(l) if l == size => { std::str::from_utf8(contents); contents as ~str }
    //     _ => fail!("Could not read file\n")
    // }
    let contents = f.read_to_end();
    std::str::from_utf8_owned(contents)
}

fn open_or_fail(name: &str) -> File {
    match File::open(&Path::new(name)) {
        Some(f) => f,
        None => fail!("Could not open file\n")
    }
}
