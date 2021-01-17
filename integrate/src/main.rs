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

fn main() {
    let vm_path = "integrate/src/programs/Add.vm";
    let asm_path = "integrate/src/programs/Add.asm";
    let ml_path = "integrate/src/programs/Add.txt";

    // VM to Assembly
    let mut vm_translator = VmTranslator::new();
    vm_translator.run(vm_path, asm_path).expect("Translate VM");

    // Assembly to Machine language(ML)
    let mut parser = Parser::new(); // TODO: include saving feature ?
    let parsed = parser.run(asm_path);
    let mut file = File::create(ml_path).unwrap();
    file.write_all(parsed.to_string().as_bytes()).unwrap();

    // Execute ml
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        match stream {
            Err(e) => panic!(e),
            Ok(tcp) => {
                tcp.set_nonblocking(true).unwrap();
                let socket = accept(tcp).unwrap();
                start_computer(socket, ml_path);
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
