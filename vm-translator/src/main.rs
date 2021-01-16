use vm_translator::VmTranslator;

mod code_writer;
mod parser;
mod vm_translator;

fn main() {
    let mut vm_translator = VmTranslator::new();
    let res = vm_translator.run("src/tests/SimpleAdd.vm");
    println!("{:?}", res);
}
