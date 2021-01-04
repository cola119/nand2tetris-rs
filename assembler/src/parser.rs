#![allow(dead_code)]
use std::{
    fmt::{self, Formatter},
    fs::File,
    io::BufRead,
    io::BufReader,
    write,
};
use CommandType::{ACommand, CCommand, LCommand};

use crate::{
    code::{comp_map, dest_map, jump_map},
    symbol_table::SymbolTable,
};

#[derive(Debug)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

#[derive(Debug)]
pub struct HackToken {
    ctype: CommandType,
    symbol: Option<String>,
    dest: Option<String>,
    comp: Option<String>,
    jump: Option<String>,
}

impl HackToken {
    pub fn c_cmd(
        ctype: CommandType,
        dest: Option<String>,
        comp: Option<String>,
        jump: Option<String>,
    ) -> Self {
        Self {
            ctype,
            symbol: None,
            dest: dest,
            comp: comp,
            jump: jump,
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

#[derive(Debug)]
pub struct ParseResult {
    tokens: Vec<HackToken>,
    symbol_table: SymbolTable,
}

impl fmt::Display for ParseResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = self
            .tokens
            .iter()
            .fold("".to_string(), |acc, token| -> String {
                let binary_str = match token.ctype {
                    CCommand => format!(
                        "111{}{}{}",
                        token.comp.as_ref().unwrap(),
                        token.dest.as_ref().unwrap(),
                        token.jump.as_ref().unwrap()
                    ),
                    ACommand | LCommand => {
                        let symbol = token.symbol.as_ref().unwrap();
                        if symbol.starts_with("0") || symbol.starts_with("1") {
                            format!("0{}", symbol)
                        } else {
                            let addr = self
                                .symbol_table
                                .get_address(&symbol)
                                .expect(&format!("couldn't find address of symbol: {}", symbol));
                            format!("0{}", addr)
                        }
                    }
                };
                if acc == "" {
                    return format!("{}", binary_str);
                } else {
                    return format!("{}\n{}", acc, binary_str);
                }
            });
        write!(f, "{}", str)
    }
}

#[derive(Debug)]
pub struct Parser {
    lines: Vec<String>,
    index: usize,
    command: Option<String>,
    symbol_table: SymbolTable,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            index: 0,
            command: None,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn run(&mut self, filename: &str) -> ParseResult {
        self.load(filename);
        self.init_symbol_table();

        let mut tokens: Vec<HackToken> = Vec::new();

        while self.has_more_commands() {
            self.advance();
            if self.command == None {
                continue;
            }
            let ctype = self.command_type();
            match ctype {
                ACommand | LCommand => {
                    tokens.push(HackToken::a_or_l_cmd(ctype, self.symbol()));
                }
                CCommand => {
                    tokens.push(HackToken::c_cmd(
                        ctype,
                        self.dest(),
                        Some(self.comp()),
                        self.jump(),
                    ));
                }
            }
        }

        ParseResult {
            tokens,
            symbol_table: self.symbol_table.clone(),
        }
    }

    fn load(&mut self, filename: &str) {
        let file = File::open(filename).expect(&format!("Failed to open {}", filename));
        self.lines = BufReader::new(file)
            .lines()
            .map(|line| -> String { line.unwrap().to_string() })
            .collect()
    }

    fn init_symbol_table(&mut self) {
        let mut line_count = 0;
        while self.has_more_commands() {
            self.advance();
            if self.command == None {
                continue;
            }
            match self.command_type() {
                ACommand | CCommand => {
                    line_count += 1;
                }
                LCommand => {
                    let symbol = &self.symbol();
                    let address = &format!("{:015b}", line_count + 1);
                    self.symbol_table.add_entry(symbol, address);
                }
            }
        }
        self.index = 0;
        self.command = None;
    }

    fn has_more_commands(&self) -> bool {
        self.lines.len() > self.index
    }

    fn advance(&mut self) {
        let cmd = self.lines.get(self.index);
        self.command = cmd.map_or(None, |str| {
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

    fn command_type(&self) -> CommandType {
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

    fn symbol(&mut self) -> String {
        let cmd = self.command.as_ref().unwrap();
        if cmd.starts_with("@") {
            let symbol = cmd.split("@").nth(1).unwrap().to_string();
            if let Ok(symbol_num) = symbol.parse::<i32>() {
                // @30
                println!("{}", symbol_num);
                return format!("{:015b}", symbol_num);
            }
            // @LOOP
            let addr_or_not = self.symbol_table.get_address(&symbol);
            if addr_or_not == None {
                // 変数として割当
                self.symbol_table.insert_variable_symbol(&symbol);
            }
            return symbol;
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

    fn comp(&self) -> String {
        let cmd = self.command.as_ref().unwrap();
        if cmd.contains("=") {
            let inst = cmd.split("=").nth(1).unwrap();
            return comp_map(inst).to_string();
        } else if cmd.contains(";") {
            let inst = cmd.split(";").nth(0).unwrap();
            return comp_map(inst).to_string();
        }
        panic!(format!("unknown command: {}", cmd));
    }
    fn dest(&self) -> Option<String> {
        let cmd = self.command.as_ref().unwrap();
        if cmd.contains("=") {
            let inst = cmd.split("=").nth(0);
            return Some(dest_map(inst).to_string());
        }
        return Some(dest_map(None).to_string());
    }
    fn jump(&self) -> Option<String> {
        let cmd = self.command.as_ref().unwrap();
        if cmd.contains(";") {
            let inst = cmd.split(";").nth(1);
            return Some(jump_map(inst).to_string());
        }
        return Some(jump_map(None).to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_parser1() {
        let mut parser = Parser::new();
        let result = parser.run("src/programs/Add.asm");
        assert_eq!(
            result.to_string(),
           "0000000000000010\n1110110000010000\n0000000000000011\n1110000010010000\n0000000000000000\n1110001100001000"
        )
    }
}
