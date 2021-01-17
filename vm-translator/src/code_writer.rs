#![allow(dead_code)]
use crate::parser::VmCommandType::{ARITHMETIC, PUSH};
use crate::parser::VmToken;

pub struct VmCodeWriter {}

impl VmCodeWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn translate(&self, token: &VmToken) -> String {
        match token.ctype {
            ARITHMETIC => self.translate_arithmetic(token.arg1.as_ref().unwrap()),
            PUSH => self.translate_push(token.arg1.as_ref().unwrap(), token.arg2.as_ref().unwrap()),
            _ => {
                panic!("WIP");
            }
        }
    }

    fn translate_push(&self, arg1: &str, arg2: &str) -> String {
        let set_arg2 = format!("@{}\nD=A", arg2);
        let code = match arg1 {
            "constant" => set_arg2,
            "argument" => format!("{}\n@ARG\nA=D+M\nD=M", set_arg2),
            _ => panic!(format!("Unknown arg1: {}, arg2: {}", arg1, arg2)),
        };
        let incr_sp = "@SP\nA=M\nM=D\n@SP\nM=M+1";
        format!("{}\n{}", code, incr_sp)
    }

    fn translate_arithmetic(&self, operator_str: &str) -> String {
        let operator = match operator_str {
            "add" => "+",
            "sub" => "-",
            _ => panic!(format!("Unknown operator: {}", operator_str)),
        };
        format!(
            "@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\nD=D{}M\n@SP\nA=M\nM=D\n@SP\nM=M+1",
            operator
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_writer_1() {
        let writer = VmCodeWriter::new();
        let token = VmToken {
            ctype: PUSH,
            arg1: Some("constant".to_string()),
            arg2: Some("1".to_string()),
        };
        assert_eq!(
            writer.translate(&token),
            "@1
            D=A
            @SP
            A=M
            M=D
            @SP
            M=M+1"
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("\n")
        );
    }

    #[test]
    fn for_writer_2() {
        let writer = VmCodeWriter::new();
        let token = VmToken {
            ctype: PUSH,
            arg1: Some("argument".to_string()),
            arg2: Some("1".to_string()),
        };
        assert_eq!(
            writer.translate(&token),
            "@1
            D=A
            @ARG
            A=D+M
            D=M
            @SP
            A=M
            M=D
            @SP
            M=M+1"
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("\n")
        );
    }

    #[test]
    fn for_writer_3() {
        let writer = VmCodeWriter::new();
        let token = VmToken {
            ctype: ARITHMETIC,
            arg1: Some("add".to_string()),
            arg2: None,
        };
        assert_eq!(
            writer.translate(&token),
            "@SP
            M=M-1
            A=M
            D=M
            @SP
            M=M-1
            A=M
            D=D+M
            @SP
            A=M
            M=D
            @SP
            M=M+1"
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("\n")
        );
    }
}
