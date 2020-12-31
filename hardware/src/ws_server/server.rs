use std::net::{TcpListener, TcpStream};
use tungstenite::{server::accept, Message, WebSocket};

#[derive(Debug)]
pub struct WsServer {
    socket: Option<WebSocket<TcpStream>>,
}

impl WsServer {
    pub fn new() -> Self {
        Self { socket: None }
    }

    pub fn run(&mut self) {
        let server = TcpListener::bind("127.0.0.1:9001").unwrap();
        for stream in server.incoming() {
            println!("socket incoming");
            self.socket = accept(stream.unwrap()).ok();
            // let msg = self
            //     .socket
            //     .expect("socket not found")
            //     .read_message()
            //     .unwrap();
            // let text = msg.into_text().unwrap().clone();
            // println!("got message: {:?}", text);
        }
    }

    pub fn write(&mut self, msg: String) {
        println!("write {:?}", msg);
        match self.socket {
            Some(ref mut x) => {
                x.write_message(Message::from(msg)).expect("send error");
            }
            None => {
                println!("none");
            }
        };
    }
}
