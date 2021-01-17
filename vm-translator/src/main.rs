use vm_translator::VmTranslator;

mod code_writer;
mod parser;
mod vm_translator;

fn main() {
    let mut vm_translator = VmTranslator::new();
    vm_translator
        .run("src/programs/SimpleAdd.vm", "src/programs/SimpleAdd.asm")
        .unwrap();
}

// initialize stack ?
