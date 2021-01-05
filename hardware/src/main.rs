mod base;
mod computer;
mod util;

use base::logic::bit::O;
use base::screen::ScreenWriter;
use computer::Computer;
use std::net::{TcpListener, TcpStream};
use tungstenite::{server::accept, WebSocket};

fn start_computer(socket: WebSocket<TcpStream>) {
    println!("start_computer");
    let writer = ScreenWriter::new(socket);

    let mut computer = Computer::new(Some(writer), false);

    computer.run("src/program/rect_from_assembler.txt", false);

    println!(
        "{}",
        computer.memory_out([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
    );

    println!("done");
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        println!("socket incoming");
        let socket = accept(stream.unwrap()).unwrap();
        start_computer(socket);
    }
}
