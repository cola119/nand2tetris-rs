#![allow(dead_code)]
extern crate assembler;
extern crate hardware;
extern crate vm_translator;
use assembler::parser::Parser;
use hardware::computer::Computer;
use vm_translator::vm_translator::VmTranslator;

use std::fs::File;
use std::io::Write;
use std::process;
use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
};
use tungstenite::{server::accept, Message, WebSocket};

struct VmScanner {
    vm_path: String,
    asm_path: String,
    pub ml_path: String,
}

impl VmScanner {
    pub fn new(program_path: &str) -> Self {
        Self {
            vm_path: format!("{}.vm", program_path),
            asm_path: format!("{}.asm", program_path),
            ml_path: format!("{}.txt", program_path),
        }
    }

    pub fn run(&self) {
        self.vm_to_assembly();
        self.assembly_to_ml();
    }

    fn vm_to_assembly(&self) {
        let mut vm_translator = VmTranslator::new();
        vm_translator
            .run(&self.vm_path, &self.asm_path)
            .expect("Translate VM");
    }

    fn assembly_to_ml(&self) {
        let mut parser = Parser::new(); // TODO: include saving feature ?
        let parsed = parser.run(&self.asm_path);
        let mut file = File::create(&self.ml_path).unwrap();
        file.write_all(parsed.to_string().as_bytes()).unwrap();
    }
}

fn main() {
    let scanner = VmScanner::new("integrate/src/programs/Add");
    scanner.run();

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        match stream {
            Err(e) => panic!(e),
            Ok(tcp) => {
                tcp.set_nonblocking(true).unwrap();
                let socket = accept(tcp).unwrap();
                start_computer(socket, &scanner.ml_path);
            }
        }
    }
}

fn start_computer(mut socket: WebSocket<TcpStream>, filename: &str) {
    println!("------ start_computer ------");

    let (to_computer, from_external) = mpsc::channel::<String>();
    let (to_external, from_computer) = mpsc::channel::<String>();
    // issue: CPU usage hits 100%
    thread::spawn(move || loop {
        if let Ok(msg) = socket.read_message() {
            to_computer.send(msg.to_string()).unwrap();
        }
        if let Ok(msg) = from_computer.try_recv() {
            socket.write_message(Message::from(msg)).unwrap();
        }
    });

    let mut computer = Computer::new(Some((to_external, from_external)), false);

    computer.run(filename, false);

    println!("{}", computer.get_memory_info(0, 8));
    println!("{}", computer.get_memory_info(256, 260));

    println!("------ start_stop ------");
    process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use hardware::base::logic::Word;

    #[test]
    fn integrate_test_add() {
        let scanner = VmScanner::new("src/programs/Add");
        scanner.run();
        let mut computer = Computer::new(None, false);
        computer.run(&scanner.ml_path, false);
        assert_eq!(
            computer.memory_out("000000100000000"),
            Word::from("0000000000001111")
        );
    }

    #[test]
    fn integrate_test_sub() {
        let scanner = VmScanner::new("src/programs/Sub");
        scanner.run();
        let mut computer = Computer::new(None, false);
        computer.run(&scanner.ml_path, false);
        assert_eq!(
            computer.memory_out("000000100000000"),
            Word::from("1111111111111110")
        );
    }

    #[test]
    fn integrate_test_eq() {
        let scanner = VmScanner::new("src/programs/Eq");
        scanner.run();
        let mut computer = Computer::new(None, false);
        computer.run(&scanner.ml_path, false);
        assert_eq!(
            computer.memory_out("000000100000000"),
            Word::from("1111111111111111")
        );
    }

    #[test]
    fn integrate_test_lt() {
        let scanner = VmScanner::new("src/programs/Lt");
        scanner.run();
        let mut computer = Computer::new(None, false);
        computer.run(&scanner.ml_path, false);
        assert_eq!(
            computer.memory_out("000000100000000"),
            Word::from("1111111111111111")
        );
    }

    #[test]
    fn integrate_test_gt() {
        let scanner = VmScanner::new("src/programs/Gt");
        scanner.run();
        let mut computer = Computer::new(None, false);
        computer.run(&scanner.ml_path, false);
        assert_eq!(
            computer.memory_out("000000100000000"),
            Word::from("0000000000000000")
        );
    }
}
