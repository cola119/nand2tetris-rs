#![allow(dead_code)]
use std::fs::File;
use std::io::Write;

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

    pub fn run(&mut self, input_file: &str, output_file: &str) -> Result<(), std::io::Error> {
        let output = self.translate(input_file);
        let mut file = File::create(output_file).unwrap();
        file.write_all(output.as_bytes())
    }

    pub fn translate(&mut self, input_file: &str) -> String {
        let mut code = "@256\nD=A\n@SP\nM=D".to_string();
        let parsed = self.parser.run(input_file);
        for token in parsed.tokens.iter() {
            let sub_code = self.writer.translate(token);
            code = format!("{}\n{}", code, sub_code)
        }
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_translator_1() {
        let mut translator = VmTranslator::new();
        let result = translator.translate("src/tests/add.vm");
        let expect = "@256
                            D=A
                            @SP
                            M=D
                            @7
                            D=A
                            @SP
                            A=M
                            M=D
                            @SP
                            M=M+1
                            @8
                            D=A
                            @SP
                            A=M
                            M=D
                            @SP
                            M=M+1
                            @SP
                            AM=M-1
                            D=M
                            @SP
                            AM=M-1
                            D=D+M
                            @SP
                            A=M
                            M=D
                            @SP
                            M=M+1"
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("\n");
        assert_eq!(result, expect);
    }
}
