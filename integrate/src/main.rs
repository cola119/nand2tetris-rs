extern crate assembler;
extern crate vm_translator;
use assembler::parser::Parser;
use vm_translator::vm_translator::VmTranslator;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut vm_translator = VmTranslator::new();
    vm_translator
        .run(
            "integrate/src/programs/Add.vm",
            "integrate/src/programs/Add.asm",
        )
        .expect("Translate VM");

    let mut parser = Parser::new();
    // TODO: include saving feature ?
    let parsed = parser.run("integrate/src/programs/Add.asm");
    let mut file = File::create("integrate/src/programs/Add.txt").unwrap();
    file.write_all(parsed.to_string().as_bytes()).unwrap();
}
