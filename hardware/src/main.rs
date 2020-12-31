mod base;

use base::screen::Screen;
use base::{dff::Clock, logic::Word};
use base::{
    logic::bit::{I, O},
    screen::ScreenWriter,
};
use std::net::{TcpListener, TcpStream};
use tungstenite::{server::accept, WebSocket};

fn start_computer(socket: WebSocket<TcpStream>) {
    println!("start_computer");
    let mut clock = Clock::new();
    let writer = ScreenWriter::new(socket);
    let mut screen = Screen::new(Some(writer));

    screen.input(
        &clock,
        Word::new([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
        I,
        [I, I, I, I, I, I, I, I, I, I, I, I, I],
    );
    clock.next();
    clock.next();

    screen.input(
        &clock,
        Word::new([O, O, O, I, O, I, O, O, O, I, O, I, O, O, O, O]),
        I,
        [O, I, I, O, O, O, O, O, O, O, I, O, O],
    );
    clock.next();
    clock.next();

    screen.input(
        &clock,
        Word::new([O, O, O, I, I, I, I, I, I, I, I, I, I, O, O, O]),
        I,
        [O, O, O, O, O, I, O, O, O, O, O, I, O],
    );
    clock.next();
    clock.next();
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        println!("socket incoming");
        let socket = accept(stream.unwrap()).unwrap();
        start_computer(socket);
    }
}
