use std::fs::File;
use std::io::Write;

use parser::Parser;

mod code;
mod parser;
mod symbol_table;
mod util;

fn main() {
    let mut parser = Parser::new();
    let result = parser.run("src/programs/test.asm");

    let mut file = File::create("src/programs/test.txt").unwrap();
    file.write_all(result.to_string().as_bytes()).unwrap();
}
