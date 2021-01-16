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

#[derive(Debug)]
pub struct Parser {
    lines: Vec<String>,
    index: usize,
    // push local 1
    command: Option<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            index: 0,
            command: None,
        }
    }

    pub fn run(&mut self, filename: &str) {
        self.load(filename);
        while self.has_more_commands() {
            self.advance();
            if self.command == None {
                continue;
            }
            println!("{:?}", self.command);
            println!("{:?}", self.command_type());

            if self.command_type() != RETURN {
                println!("{:?}", self.arg1());
            }
            if self.command_type() == PUSH
                || self.command_type() == POP
                || self.command_type() == FUNCTION
                || self.command_type() == CALL
            {
                println!("{:?}", self.arg2());
            }
        }
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
        let mut parser = Parser::new();
        parser.run("src/tests/vm_list.vm");
    }
}
