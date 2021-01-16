#![allow(dead_code)]
use crate::{code_writer::VmCodeWriter, parser::VmParser};

pub struct VmTranslator {
    parser: VmParser,
    writer: VmCodeWriter,
}

impl VmTranslator {
    pub fn new() -> Self {
        Self {
            parser: VmParser::new(),
            writer: VmCodeWriter::new(),
        }
    }

    pub fn run(&mut self, input_file: &str) -> String {
        let mut code = "".to_string();
        let parsed = self.parser.run(input_file);
        for token in parsed.tokens.iter() {
            let sub_code = self.writer.translate(token);
            code = if code == "" {
                sub_code
            } else {
                format!("{}\n{}", code, sub_code)
            }
        }
        // TODO save
        code
    }
}
