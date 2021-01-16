mod base;
mod computer;
mod util;

use computer::Computer;
use std::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
};
use tungstenite::{server::accept, Message, WebSocket};

fn start_computer(mut socket: WebSocket<TcpStream>) {
    println!("start_computer");

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

    computer.run("src/program/pong.txt", false);

    println!("{}", computer.memory_out("000000000000000"));
    println!("{}", computer.memory_out("000000100000000"));

    println!("done");
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        match stream {
            Err(e) => panic!(e),
            Ok(tcp) => {
                tcp.set_nonblocking(true).unwrap();
                let socket = accept(tcp).unwrap();
                start_computer(socket);
            }
        }
    }
}
