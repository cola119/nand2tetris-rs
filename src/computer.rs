#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    dff::Clock,
    logic::bit::{I, O},
    logic::{bit, dmux8way, mux4way16, mux8way16, Word},
    ram::RAM4K,
};

pub struct ROM32K {
    rams: [RAM4K; 8],
}

impl ROM32K {
    pub fn new() -> Self {
        Self {
            rams: [RAM4K::new(); 8],
        }
    }

    // the same as RAMn
    fn input_to_rams(&mut self, clock_t: &Clock, input: Word, address: [bit; 15]) {
        let load_bit = dmux8way(I, [address[0], address[1], address[2]]);
        let register_addr = [
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
            address[13],
            address[14],
        ];

        self.rams[0].input(clock_t, input, register_addr, load_bit[0]);
        self.rams[1].input(clock_t, input, register_addr, load_bit[1]);
        self.rams[2].input(clock_t, input, register_addr, load_bit[2]);
        self.rams[3].input(clock_t, input, register_addr, load_bit[3]);
        self.rams[4].input(clock_t, input, register_addr, load_bit[4]);
        self.rams[5].input(clock_t, input, register_addr, load_bit[5]);
        self.rams[6].input(clock_t, input, register_addr, load_bit[6]);
        self.rams[7].input(clock_t, input, register_addr, load_bit[7]);
    }

    // 読み取り専用
    pub fn output(&mut self, clock_t: &Clock, address: [bit; 15]) -> Word {
        let register_address = [
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
            address[13],
            address[14],
        ];
        mux8way16(
            self.rams[0].output(clock_t, register_address),
            self.rams[1].output(clock_t, register_address),
            self.rams[2].output(clock_t, register_address),
            self.rams[3].output(clock_t, register_address),
            self.rams[4].output(clock_t, register_address),
            self.rams[5].output(clock_t, register_address),
            self.rams[6].output(clock_t, register_address),
            self.rams[7].output(clock_t, register_address),
            [address[0], address[1], address[2]],
        )
    }

    pub fn load(&mut self, filename: &str) {
        let clock_t = Clock::new();
        let file = File::open(filename).expect(&format!("Fail to open {}", filename));
        for line_result in BufReader::new(file).lines() {
            let line: &str = &line_result.expect("couldn't read lines");
            let instruction = Word::from(line);
            let address = [
                instruction[1],
                instruction[2],
                instruction[3],
                instruction[4],
                instruction[5],
                instruction[6],
                instruction[7],
                instruction[8],
                instruction[9],
                instruction[10],
                instruction[11],
                instruction[12],
                instruction[13],
                instruction[14],
                instruction[15],
            ];
            self.input_to_rams(&clock_t, instruction, address);
        }
        // load=0で仮にinputを呼ぶことでbitのstateをcurrentからprevに移す
        for i in 0..8 {
            self.rams[i].input(&clock_t, Word::new([O; 16]), [I; 12], O);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ROM32K;
    use crate::logic::bit::{I, O};
    use crate::{dff::Clock, logic::Word};

    #[test]
    fn for_rom_load() {
        let clock = Clock::new();
        let mut rom = ROM32K::new();
        rom.load("src/tests/sample.txt");

        assert_eq!(rom.output(&clock, [O; 15]), Word::new([O; 16]));
        assert_eq!(
            rom.output(&clock, [O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I])
        );
        assert_eq!(
            rom.output(&clock, [O, O, O, O, O, O, O, O, O, O, I, O, O, O, I]),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, I, O, O, O, I])
        );
        assert_eq!(
            rom.output(&clock, [O, O, O, O, I, I, I, I, I, I, I, I, I, I, I]),
            Word::new([O, O, O, O, O, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            // 000,111111111111 => register[0][2047]
            rom.output(&clock, [O, O, O, I, I, I, I, I, I, I, I, I, I, I, I]),
            Word::new([O, O, O, O, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            // 001,111111111111 => register[1][2047]
            rom.output(&clock, [O, O, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            Word::new([O, O, O, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            // 011,111111111111 => register[3][2047]
            rom.output(&clock, [O, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            Word::new([O, O, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            // instruction => 0,111111111111111
            // 1,111,11111111111 => register[7][2047]
            rom.output(&clock, [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            Word::new([O, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
        assert_eq!(
            // instruction => 1,111111111111111
            // 1,111,11111111111 => register[7][2047]
            rom.output(&clock, [I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
            // instruction => 0,111111111111111と同じ
            Word::new([O, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I])
        );
    }
}
