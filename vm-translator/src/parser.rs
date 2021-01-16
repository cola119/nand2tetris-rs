#![allow(dead_code)]
use std::{fs::File, io::BufRead, io::BufReader};
use VmCommandType::{ARITHMETIC, CALL, FUNCTION, GOTO, IF, LABEL, POP, PUSH, RETURN};
#[derive(Debug, PartialEq)]
enum VmCommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
}

#[derive(Debug, PartialEq)]
pub struct VmParserResult {
    tokens: Vec<VmToken>,
}

#[derive(Debug, PartialEq)]
pub struct VmToken {
    ctype: VmCommandType,
    arg1: Option<String>,
    arg2: Option<String>,
}

#[derive(Debug)]
pub struct VmParser {
    lines: Vec<String>,
    index: usize,
    // push local 1
    command: Option<String>,
}

impl VmParser {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            index: 0,
            command: None,
        }
    }

    pub fn run(&mut self, filename: &str) -> VmParserResult {
        let mut tokens: Vec<VmToken> = Vec::new();

        self.load(filename);
        while self.has_more_commands() {
            self.advance();
            if self.command == None {
                continue;
            }

            let ctype = self.command_type();
            let token = match ctype {
                RETURN => VmToken {
                    ctype,
                    arg1: None,
                    arg2: None,
                },
                PUSH | POP | FUNCTION | CALL => VmToken {
                    ctype,
                    arg1: Some(self.arg1()),
                    arg2: Some(self.arg2()),
                },
                ARITHMETIC | LABEL | GOTO | IF => VmToken {
                    ctype,
                    arg1: Some(self.arg1()),
                    arg2: None,
                },
                // _ => {
                //     panic!(format!("Unknown command: {:?}", self.command))
                // }
            };

            tokens.push(token);
        }

        return VmParserResult { tokens };
    }

    fn load(&mut self, filename: &str) {
        let file = File::open(filename).expect(&format!("Failed to open {:?}", filename));
        self.lines = BufReader::new(file)
            .lines()
            .map(|line| -> String { line.unwrap().to_string() })
            .collect();
    }

    fn has_more_commands(&self) -> bool {
        self.lines.len() > self.index
    }

    fn advance(&mut self) {
        let line = self.lines.get(self.index);
        self.command = line.map_or(None, |str| {
            let trmed = str.trim();
            if trmed.starts_with("//") || trmed == "" {
                return None;
            }
            trmed
                .split("//")
                .nth(0)
                .map(|s| -> String { s.trim().to_string() })
        });
        self.index += 1;
    }

    fn command_type(&self) -> VmCommandType {
        let str: &str = &self.command.as_ref().unwrap();
        let command_str = str
            .split(" ")
            .nth(0)
            .expect(&format!("cannot parse command: {}", str));
        match command_str {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => ARITHMETIC,
            "push" => PUSH,
            "pop" => POP,
            "label" => LABEL,
            "goto" => GOTO,
            "if-goto" => IF,
            "function" => FUNCTION,
            "call" => CALL,
            "return" => RETURN,
            _ => panic!(format!("unknown command: {}", command_str)),
        }
    }

    fn arg1(&self) -> String {
        let str: &str = &self.command.as_ref().unwrap();
        let parsed = str.split(" ").collect::<Vec<&str>>();
        if self.command_type() == ARITHMETIC {
            parsed.get(0).unwrap().to_string()
        } else {
            parsed.get(1).unwrap().to_string()
        }
    }

    fn arg2(&self) -> String {
        let str: &str = &self.command.as_ref().unwrap();
        let parsed = str.split(" ").collect::<Vec<&str>>();
        parsed.get(2).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_parser_1() {
        let mut parser = VmParser::new();
        let result = parser.run("src/tests/vm_list.vm");

        let expect = VmParserResult {
            tokens: Vec::from([
                VmToken {
                    ctype: ARITHMETIC,
                    arg1: Some("add".to_string()),
                    arg2: None,
                },
                VmToken {
                    ctype: PUSH,
                    arg1: Some("local".to_string()),
                    arg2: Some("1".to_string()),
                },
                VmToken {
                    ctype: POP,
                    arg1: Some("local".to_string()),
                    arg2: Some("1".to_string()),
                },
                VmToken {
                    ctype: LABEL,
                    arg1: Some("label_arg".to_string()),
                    arg2: None,
                },
                VmToken {
                    ctype: GOTO,
                    arg1: Some("goto_arg".to_string()),
                    arg2: None,
                },
                VmToken {
                    ctype: IF,
                    arg1: Some("if-goto_arg".to_string()),
                    arg2: None,
                },
                VmToken {
                    ctype: FUNCTION,
                    arg1: Some("functionName".to_string()),
                    arg2: Some("nLocals".to_string()),
                },
                VmToken {
                    ctype: CALL,
                    arg1: Some("functionName".to_string()),
                    arg2: Some("nArgs".to_string()),
                },
                VmToken {
                    ctype: RETURN,
                    arg1: None,
                    arg2: None,
                },
            ]),
        };

        assert_eq!(result, expect);
    }
}
