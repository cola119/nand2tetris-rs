#![allow(dead_code)]
use crate::parser::VmCommandType::{ARITHMETIC, PUSH};
use crate::parser::VmToken;

pub struct VmCodeWriter {
    label_cnt: i32,
}

impl VmCodeWriter {
    pub fn new() -> Self {
        Self { label_cnt: -1 }
    }

    pub fn translate(&mut self, token: &VmToken) -> String {
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

    fn translate_arithmetic(&mut self, operator_str: &str) -> String {
        let incr_and_push = "@SP\nA=M\nM=D\n@SP\nM=M+1";
        let mut get_label_cnt = || {
            self.label_cnt += 1;
            self.label_cnt
        };
        let formula = match operator_str {
            "add" => format!("D=D+M\n{}", incr_and_push),
            "sub" => format!("D=M-D\n{}", incr_and_push),
            "neg" => format!("D=-D\n{}", incr_and_push),
            "and" => format!("D=D&M\n{}", incr_and_push),
            "or" => format!("D=D|M\n{}", incr_and_push),
            "not" => format!("D=!D\n{}", incr_and_push),
            "eq" | "gt" | "lt" => {
                let cnt = get_label_cnt();
                let true_label = format!("TRUE_LB_{}", cnt);
                let false_label = format!("FALSE_LB_{}", cnt);
                match operator_str {
                    "eq" => format!(
                        "D=M-D\n@{}\nD;JEQ\nD=0\n{}\n@{}\n0;JMP\n({})\nD=-1\n{}\n({})\n@SP",
                        true_label,
                        incr_and_push,
                        false_label,
                        true_label,
                        incr_and_push,
                        false_label
                    ),
                    "gt" => format!(
                        "D=M-D\n@{}\nD;JGT\nD=0\n{}\n@{}\n0;JMP\n({})\nD=-1\n{}\n({})\n@SP",
                        true_label,
                        incr_and_push,
                        false_label,
                        true_label,
                        incr_and_push,
                        false_label
                    ),
                    "lt" => format!(
                        "D=M-D\n@{}\nD;JLT\nD=0\n{}\n@{}\n0;JMP\n({})\nD=-1\n{}\n({})\n@SP",
                        true_label,
                        incr_and_push,
                        false_label,
                        true_label,
                        incr_and_push,
                        false_label
                    ),
                    _ => panic!(format!("Unknown operator: {}", operator_str)),
                }
            }
            _ => panic!(format!("Unknown operator: {}", operator_str)),
        };
        format!("@SP\nM=M-1\nA=M\nD=M\n@SP\nM=M-1\nA=M\n{}", formula)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_writer_1() {
        let mut writer = VmCodeWriter::new();
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
        let mut writer = VmCodeWriter::new();
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
        let mut writer = VmCodeWriter::new();
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

    #[test]
    fn for_writer_4() {
        let mut writer = VmCodeWriter::new();
        let token = VmToken {
            ctype: ARITHMETIC,
            arg1: Some("eq".to_string()),
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
            D=M-D
            @TRUE_LB_0
            D;JEQ
            D=0
            @SP
            A=M
            M=D
            @SP
            M=M+1
            @FALSE_LB_0
            0;JMP
            (TRUE_LB_0)
            D=-1
            @SP
            A=M
            M=D
            @SP
            M=M+1
            (FALSE_LB_0)
            @SP"
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("\n")
        );
    }
}
