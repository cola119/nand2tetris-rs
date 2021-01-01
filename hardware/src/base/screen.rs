#![allow(dead_code)]
use std::net::TcpStream;

use tungstenite::{Message, WebSocket};

use crate::base::logic::{
    bit::{self, I, O},
    dmux, mux, Word,
};
use crate::base::{dff::Clock, ram::RAM4K};

#[derive(Debug)]
pub struct ScreenWriter {
    ws: WebSocket<TcpStream>,
}
impl ScreenWriter {
    pub fn new(socket: WebSocket<TcpStream>) -> Self {
        Self { ws: socket }
    }
    pub fn write_msg(&mut self, msg: String) {
        self.ws
            .write_message(Message::from(msg))
            .expect("couldn't send")
    }
}

#[derive(Debug)]
pub struct Screen {
    rams: [RAM4K; 2],
    writer: Option<ScreenWriter>,
}

impl Screen {
    pub fn new(writer: Option<ScreenWriter>) -> Self {
        Self {
            rams: [RAM4K::new(); 2],
            writer,
        }
    }

    pub fn input(&mut self, clock_t: &Clock, input: Word, address: [bit; 13], load: bit) {
        let ram_addr = [
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
        ];
        let load_bits = dmux(load, address[0]);
        self.rams[0].input(clock_t, input, ram_addr, load_bits[0]);
        self.rams[1].input(clock_t, input, ram_addr, load_bits[1]);

        self.send_message(input, address);
    }

    fn send_message(&mut self, input: Word, address: [bit; 13]) {
        let digit = 13;
        let register_index = (0..digit).fold(0, |sum, i| {
            sum + (address[i].to_string().parse::<usize>().unwrap())
                * 2usize.pow((digit - 1 - i) as u32)
        });
        let y = register_index / 32;
        let x = register_index % 32;
        let message = format!(
            "{{\"register_index\":{},\"x\":{},\"y\":{},\"input\":\"{}\"}}",
            register_index, x, y, input
        );
        match self.writer {
            Some(ref mut x) => x.write_msg(message),
            None => {
                println!("none");
            }
        };
    }

    pub fn output(&self, clock_t: &Clock, address: [bit; 13]) -> Word {
        let ram_addr = [
            address[1],
            address[2],
            address[3],
            address[4],
            address[5],
            address[6],
            address[7],
            address[8],
            address[9],
            address[10],
            address[11],
            address[12],
        ];
        let out1 = self.rams[0].output(clock_t, ram_addr);
        let out2 = self.rams[1].output(clock_t, ram_addr);
        Word::new([
            mux(out1[0], out2[0], address[0]),
            mux(out1[1], out2[1], address[0]),
            mux(out1[2], out2[2], address[0]),
            mux(out1[3], out2[3], address[0]),
            mux(out1[4], out2[4], address[0]),
            mux(out1[5], out2[5], address[0]),
            mux(out1[6], out2[6], address[0]),
            mux(out1[7], out2[7], address[0]),
            mux(out1[8], out2[8], address[0]),
            mux(out1[9], out2[9], address[0]),
            mux(out1[10], out2[10], address[0]),
            mux(out1[11], out2[11], address[0]),
            mux(out1[12], out2[12], address[0]),
            mux(out1[13], out2[13], address[0]),
            mux(out1[14], out2[14], address[0]),
            mux(out1[15], out2[15], address[0]),
        ])
    }

    pub fn print(self) {
        // wip
        let clock = Clock::new();
        println!(
            "{}{}",
            self.rams[0]
                .output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O])
                .to_string(),
            self.rams[0]
                .output(&clock, [O, O, O, O, O, O, O, O, O, O, O, I])
                .to_string()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_screen() {
        let mut clock = Clock::new();
        let mut screen = Screen::new(None);
        let word1 = Word::new([I; 16]);

        screen.input(&clock, word1, [O, O, O, O, O, O, O, O, O, O, O, O, O], I);
        clock.next();
        clock.next();
        screen.input(&clock, word1, [O, O, O, O, O, O, O, O, O, O, O, O, I], I);
        assert_eq!(
            screen.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]),
            word1
        );
        clock.next();
        clock.next();
        screen.input(&clock, word1, [O, O, O, O, O, O, O, O, O, O, O, I, O], O);
        assert_eq!(
            screen.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]),
            word1
        );
        assert_eq!(
            screen.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, I]),
            word1
        );
        clock.next();
        clock.next();
        screen.input(&clock, word1, [O, O, O, O, O, O, O, O, O, O, O, I, O], O);
        assert_eq!(
            screen.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O]),
            word1
        );
        assert_eq!(
            screen.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, I]),
            word1
        );
        assert_eq!(
            screen.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, I, O]),
            Word::new([O; 16])
        );
    }
}
