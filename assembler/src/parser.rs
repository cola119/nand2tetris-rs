use std::{fs::File, io::BufRead, io::BufReader};
use CommandType::{ACommand, CCommand, LCommand};

#[derive(Debug)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

struct ParseResult {
    ctype: CommandType,
    symbol: Option<String>,
    dest: Option<String>,
    comp: Option<String>,
    jump: Option<String>,
}

impl ParseResult {
    pub fn c_cmd(ctype: CommandType, dest: String, comp: String, jump: String) -> Self {
        Self {
            ctype,
            symbol: None,
            dest: Some(dest),
            comp: Some(comp),
            jump: Some(jump),
        }
    }

    pub fn a_or_l_cmd(ctype: CommandType, symbol: String) -> Self {
        Self {
            ctype,
            symbol: Some(symbol),
            dest: None,
            comp: None,
            jump: None,
        }
    }
}

pub struct Parser {
    lines: Vec<String>,
    index: usize,
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

    pub fn run(&mut self, filename: &str) -> Vec<ParseResult> {
        self.load(filename);

        let mut result: Vec<ParseResult> = Vec::new();

        while self.has_more_commands() {
            self.advance();
            if self.command == None {
                continue;
            }
            let ctype = self.command_type();
            match ctype {
                ACommand | LCommand => {
                    result.push(ParseResult::a_or_l_cmd(ctype, self.symbol()));
                }
                CCommand => {
                    result.push(ParseResult::c_cmd(
                        ctype,
                        self.dest(),
                        self.comp(),
                        self.jump(),
                    ));
                }
            }
        }

        result
    }

    fn load(&mut self, filename: &str) {
        let file = File::open(filename).expect(&format!("Failed to open {}", filename));
        self.lines = BufReader::new(file)
            .lines()
            .map(|line| -> String { line.unwrap().to_string() })
            .collect();
        self.lines.reverse();
    }

    fn has_more_commands(&self) -> bool {
        self.lines.len() > 0
    }

    fn advance(&mut self) {
        let cmd = self.lines.pop();
        self.command = cmd.map_or(None, |str| {
            let trmed = str.trim();
            if trmed.starts_with("//") || trmed == "" {
                return None;
            }
            trmed
                .split("//")
                .nth(0)
                .map(|s| -> String { s.trim().to_string() })
        })
    }

    fn command_type(&self) -> CommandType {
        println!("{:?}", self.command);
        let str = self.command.as_ref().unwrap();
        if str.starts_with("@") {
            return ACommand;
        } else if str.contains("=") || str.contains(";") {
            return CCommand;
        } else if str.starts_with("(") && str.ends_with(")") {
            return LCommand;
        }
        panic!(format!("unknown command: {}", str));
    }

    fn symbol(&self) -> String {
        let cmd = self.command.as_ref().unwrap();
        if cmd.starts_with("@") {
            return cmd.split("@").nth(1).unwrap().to_string();
        } else if cmd.starts_with("(") {
            return cmd
                .split("(")
                .nth(1)
                .unwrap()
                .split(")")
                .nth(0)
                .unwrap()
                .to_string();
        }
        panic!(format!("unknown command: {}", cmd));
    }

    fn dest(&self) -> String {
        let cmd = self.command.as_ref().unwrap();
        if cmd.contains("=") {
            let inst = cmd.split("=").nth(0).unwrap();
        }
        "".to_string()
    }
    fn comp(&self) -> String {
        "".to_string()
    }
    fn jump(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_parser1() {
        let mut parser = Parser::new();
        let result = parser.run("src/tests/parser/1.asm");
    }
}
