use vm_translator::vm_translator::VmTranslator;

extern crate vm_translator;

fn main() {
    let mut vm_translator = VmTranslator::new();
    vm_translator
        .run(
            "integrate/src/programs/Add.vm",
            "integrate/src/programs/Add.asm",
        )
        .expect("Translate VM");
}
