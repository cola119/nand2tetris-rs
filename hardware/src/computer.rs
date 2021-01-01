#![allow(dead_code)]
use crate::base::{
    computer::ROM32K,
    cpu::CPU,
    dff::Clock,
    logic::bit::{I, O},
    logic::{and, bit, mux4way16, not, Word},
    ram::RAM16K,
    screen::{Screen, ScreenWriter},
};

pub struct Memory {
    ram: RAM16K,
    screen: Screen,
}

impl Memory {
    pub fn new(writer: Option<ScreenWriter>) -> Self {
        Self {
            ram: RAM16K::new(),
            screen: Screen::new(writer),
        }
    }

    pub fn input(&mut self, clock_t: &Clock, input: Word, address: [bit; 15], load: bit) {
        self.ram.input(
            clock_t,
            input,
            [
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
                address[13],
                address[14],
            ],
            and(not(address[0]), load),
        );
        self.screen.input(
            clock_t,
            input,
            [
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
                address[13],
                address[14],
            ],
            and(and(address[0], not(address[1])), load),
        );
    }
    pub fn output(&self, clock_t: &Clock, address: [bit; 15]) -> Word {
        let ram_out = self.ram.output(
            clock_t,
            [
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
                address[13],
                address[14],
            ],
        );
        let screen_out = self.screen.output(
            clock_t,
            [
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
                address[13],
                address[14],
            ],
        );
        // WIP
        let keyboard_out = Word::new([O; 16]);
        mux4way16(
            ram_out,
            ram_out,
            screen_out,
            keyboard_out,
            [address[0], address[1]],
        )
    }
}

pub struct Computer {
    rom: ROM32K,
    cpu: CPU,
    memory: Memory,
}

impl Computer {
    pub fn new(writer: Option<ScreenWriter>) -> Self {
        Self {
            rom: ROM32K::new(),
            cpu: CPU::new(),
            memory: Memory::new(writer),
        }
    }

    pub fn run(&mut self, filename: &str, reset: bool) {
        self.rom.load(&filename);
        let initial = self.rom.output(&Clock::new(), [O; 15]);

        let reset_bit = match reset {
            true => I,
            false => O,
        };
        self.execute(initial, Word::new([O; 16]), reset_bit);
    }

    fn execute(&mut self, instruction: Word, in_m: Word, reset: bit) {
        let mut clock = Clock::new();

        // CPU
        println!("----------------------- CPU input -----------------------");
        println!("in_memory: {}", in_m);
        println!("instruction: {}", instruction);
        println!("reset: {}", reset);

        let (out_m, write_m, address_m, pc) = self.cpu.run(&clock, in_m, instruction, reset);

        println!("----------------------- CPU output -----------------------");
        println!("out_m: {}", out_m);
        println!("write_m: {}", write_m);
        println!("address_m: {:?}", address_m);
        println!("pc: {:?}", pc);

        // Memory
        self.memory.input(&clock, out_m, address_m, write_m);
        let in_m = self.memory.output(&clock, address_m);

        // next generation
        clock.next();
        clock.next();

        // ROM
        let next_instruction = self.rom.output(&clock, pc);

        println!("");
        self.execute(next_instruction, in_m, O);
    }
}
