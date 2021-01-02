#![allow(dead_code)]
use crate::base::{
    arithmetic::add16,
    cpu::CPU,
    dff::Clock,
    logic::bit::{I, O},
    logic::{and, bit, mux4way16, not, xor, Word},
    ram::RAM16K,
    rom::ROM32K,
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
    // println
    debug: bool,
}

impl Computer {
    pub fn new(writer: Option<ScreenWriter>, debug: bool) -> Self {
        Self {
            rom: ROM32K::new(),
            cpu: CPU::new(),
            memory: Memory::new(writer),
            debug,
        }
    }

    pub fn memory_out(self, address: [bit; 15]) -> Word {
        let mut clock = Clock::new();
        clock.next();
        self.memory.output(&clock, address)
    }

    pub fn run(&mut self, filename: &str, reset: bool) {
        let instruction_num = self.rom.load(&filename);
        let reset_bit = match reset {
            true => I,
            false => O,
        };
        let initial = self.rom.output(&Clock::new(), [O; 15]);
        self.execute(
            initial,
            Word::new([O; 16]),
            reset_bit,
            instruction_num,
            Word::new([O; 16]),
        );
    }

    fn execute(
        &mut self,
        instruction: Word,
        in_m: Word,
        reset: bit,
        instruction_num: Word,
        is_last: Word,
    ) {
        let mut clock = Clock::new();

        // CPU
        if self.debug {
            println!("----------------------- CPU input -----------------------");
            println!("in_memory: {}", in_m);
            println!("instruction: {}", instruction);
            println!("reset: {}", reset);
        }

        let (out_m, write_m, address_m, pc) = self.cpu.run(&clock, in_m, instruction, reset);

        if self.debug {
            println!("----------------------- CPU output -----------------------");
            println!("out_m: {}", out_m);
            println!("write_m: {}", write_m);
            println!("address_m: {:?}", address_m);
            println!("pc: {:?}", pc);
        }

        // Memory
        self.memory.input(&clock, out_m, address_m, write_m);
        if self.debug {
            println!(
                "memory.input(input: {}, addr: {:?}, load: {})",
                out_m, address_m, write_m
            );
        }

        // next generation
        clock.next();
        clock.next();

        let in_m = self.memory.output(&clock, address_m);
        if self.debug {
            println!("{} = memory.output(addr: {:?})", in_m, address_m);
        }

        // ROM
        let next_instruction = self.rom.output(&clock, pc);

        if self.debug {
            println!("");
        }

        // 終了判定
        if is_last == Word::new([I; 16]) {
            return;
        }
        self.execute(
            next_instruction,
            in_m,
            O,
            instruction_num,
            Computer::is_last(instruction_num, pc),
        )
    }

    fn is_last(instruction_num: Word, pc: [bit; 15]) -> Word {
        let pc_16 = Word::new([
            O, pc[0], pc[1], pc[2], pc[3], pc[4], pc[5], pc[6], pc[7], pc[8], pc[9], pc[10],
            pc[11], pc[12], pc[13], pc[14],
        ]);
        let pc_incr = add16(
            pc_16,
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
        );
        Word::new([
            not(xor(instruction_num[0], pc_incr[0])),
            not(xor(instruction_num[1], pc_incr[1])),
            not(xor(instruction_num[2], pc_incr[2])),
            not(xor(instruction_num[3], pc_incr[3])),
            not(xor(instruction_num[4], pc_incr[4])),
            not(xor(instruction_num[5], pc_incr[5])),
            not(xor(instruction_num[6], pc_incr[6])),
            not(xor(instruction_num[7], pc_incr[7])),
            not(xor(instruction_num[8], pc_incr[8])),
            not(xor(instruction_num[9], pc_incr[9])),
            not(xor(instruction_num[10], pc_incr[10])),
            not(xor(instruction_num[11], pc_incr[11])),
            not(xor(instruction_num[12], pc_incr[12])),
            not(xor(instruction_num[13], pc_incr[13])),
            not(xor(instruction_num[14], pc_incr[14])),
            not(xor(instruction_num[15], pc_incr[15])),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_computer() {
        let mut computer = Computer::new(None, false);
        computer.run("src/program/max.txt", false);
        let r0 = computer.memory_out([O, O, O, O, O, O, O, O, O, O, O, O, O, I, O]);
        assert_eq!(r0, Word::from("0000000001000011"));
    }

    #[test]
    fn for_computer2() {
        let mut computer = Computer::new(None, false);
        computer.run("src/program/max2.txt", false);
        let r0 = computer.memory_out([O, O, O, O, O, O, O, O, O, O, O, O, O, I, O]);
        assert_eq!(r0, Word::from("0000000011000011"));
    }

    #[test]
    fn for_computer_add() {
        let mut computer = Computer::new(None, false);
        computer.run("src/program/add.txt", false);
        let r0 = computer.memory_out([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O]);
        assert_eq!(r0, Word::from("0000000000000101"));
    }
}
