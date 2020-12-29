#![allow(dead_code)]
use crate::dff::ClockState::{Tick, Tock};
use crate::mux16;
use crate::register::Register;
use crate::Word;
use crate::{bit, logic::mux8way16};
use crate::{bit::I, logic::dmux4way};
use crate::{bit::O, logic::dmux8way};
use crate::{dff::Clock, logic::mux4way16};

#[derive(Debug, Clone, Copy)]
pub struct RAM8 {
    registers: [Register; 8],
}

impl RAM8 {
    pub fn new() -> Self {
        Self {
            registers: [Register::new(); 8],
        }
    }
    pub fn input(&mut self, clock_t: &Clock, input: Word, address: [bit; 3], load: bit) {
        let load8 = dmux8way(load, address);
        self.registers[0].input(clock_t, input, load8[0]);
        self.registers[1].input(clock_t, input, load8[1]);
        self.registers[2].input(clock_t, input, load8[2]);
        self.registers[3].input(clock_t, input, load8[3]);
        self.registers[4].input(clock_t, input, load8[4]);
        self.registers[5].input(clock_t, input, load8[5]);
        self.registers[6].input(clock_t, input, load8[6]);
        self.registers[7].input(clock_t, input, load8[7]);
    }
    pub fn output(self, clock_t: &Clock, address: [bit; 3]) -> Word {
        mux8way16(
            self.registers[0].output(&clock_t),
            self.registers[1].output(&clock_t),
            self.registers[2].output(&clock_t),
            self.registers[3].output(&clock_t),
            self.registers[4].output(&clock_t),
            self.registers[5].output(&clock_t),
            self.registers[6].output(&clock_t),
            self.registers[7].output(&clock_t),
            address,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RAM64 {
    rams: [RAM8; 8],
}

impl RAM64 {
    pub fn new() -> Self {
        Self {
            rams: [RAM8::new(); 8],
        }
    }
    pub fn input(&mut self, clock_t: &Clock, input: Word, address: [bit; 6], load: bit) {
        let load_bit = dmux8way(load, [address[0], address[1], address[2]]);
        let register_address = [address[3], address[4], address[5]];
        self.rams[0].input(clock_t, input, register_address, load_bit[0]);
        self.rams[1].input(clock_t, input, register_address, load_bit[1]);
        self.rams[2].input(clock_t, input, register_address, load_bit[2]);
        self.rams[3].input(clock_t, input, register_address, load_bit[3]);
        self.rams[4].input(clock_t, input, register_address, load_bit[4]);
        self.rams[5].input(clock_t, input, register_address, load_bit[5]);
        self.rams[6].input(clock_t, input, register_address, load_bit[6]);
        self.rams[7].input(clock_t, input, register_address, load_bit[7]);
    }
    pub fn output(self, clock_t: &Clock, address: [bit; 6]) -> Word {
        let register_address = [address[3], address[4], address[5]];
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
}

#[derive(Debug, Clone, Copy)]
pub struct RAM512 {
    rams: [RAM64; 8],
}
impl RAM512 {
    pub fn new() -> Self {
        Self {
            rams: [RAM64::new(); 8],
        }
    }
    pub fn input(&mut self, clock_t: &Clock, input: Word, address: [bit; 9], load: bit) {
        let load_bit = dmux8way(load, [address[0], address[1], address[2]]);
        let register_address = [
            address[3], address[4], address[5], address[6], address[7], address[8],
        ];
        self.rams[0].input(clock_t, input, register_address, load_bit[0]);
        self.rams[1].input(clock_t, input, register_address, load_bit[1]);
        self.rams[2].input(clock_t, input, register_address, load_bit[2]);
        self.rams[3].input(clock_t, input, register_address, load_bit[3]);
        self.rams[4].input(clock_t, input, register_address, load_bit[4]);
        self.rams[5].input(clock_t, input, register_address, load_bit[5]);
        self.rams[6].input(clock_t, input, register_address, load_bit[6]);
        self.rams[7].input(clock_t, input, register_address, load_bit[7]);
    }
    pub fn output(self, clock_t: &Clock, address: [bit; 9]) -> Word {
        let register_address = [
            address[3], address[4], address[5], address[6], address[7], address[8],
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
}

#[derive(Debug, Clone, Copy)]
pub struct RAM4K {
    rams: [RAM512; 8],
}
impl RAM4K {
    pub fn new() -> Self {
        Self {
            rams: [RAM512::new(); 8],
        }
    }
    pub fn input(&mut self, clock_t: &Clock, input: Word, address: [bit; 12], load: bit) {
        let load_bit = dmux8way(load, [address[0], address[1], address[2]]);
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
        ];
        self.rams[0].input(clock_t, input, register_address, load_bit[0]);
        self.rams[1].input(clock_t, input, register_address, load_bit[1]);
        self.rams[2].input(clock_t, input, register_address, load_bit[2]);
        self.rams[3].input(clock_t, input, register_address, load_bit[3]);
        self.rams[4].input(clock_t, input, register_address, load_bit[4]);
        self.rams[5].input(clock_t, input, register_address, load_bit[5]);
        self.rams[6].input(clock_t, input, register_address, load_bit[6]);
        self.rams[7].input(clock_t, input, register_address, load_bit[7]);
    }
    pub fn output(self, clock_t: &Clock, address: [bit; 12]) -> Word {
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
}

#[derive(Debug, Clone, Copy)]
pub struct RAM16K {
    rams: [RAM4K; 4],
}
impl RAM16K {
    pub fn new() -> Self {
        Self {
            rams: [RAM4K::new(); 4],
        }
    }
    pub fn input(&mut self, clock_t: &Clock, input: Word, address: [bit; 14], load: bit) {
        let load_bit = dmux4way(load, [address[0], address[1]]);
        let register_address = [
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
        ];
        self.rams[0].input(clock_t, input, register_address, load_bit[0]);
        self.rams[1].input(clock_t, input, register_address, load_bit[1]);
        self.rams[2].input(clock_t, input, register_address, load_bit[2]);
        self.rams[3].input(clock_t, input, register_address, load_bit[3]);
    }
    pub fn output(self, clock_t: &Clock, address: [bit; 14]) -> Word {
        let register_address = [
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
        ];
        mux4way16(
            self.rams[0].output(clock_t, register_address),
            self.rams[1].output(clock_t, register_address),
            self.rams[2].output(clock_t, register_address),
            self.rams[3].output(clock_t, register_address),
            [address[0], address[1]],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit::{I, O};

    #[test]
    fn for_ram8() {
        // memo
        // 1クロック周期で、1inputしか許されない
        // １周期での複数回inputを回路で表現すると、同じ時間に複数のinがあることになる
        // input,outputはセット

        let zero = Word::new([O; 16]);
        let one = Word::new([I; 16]);
        let mut ram8 = RAM8::new();
        let mut clock = Clock::new();

        // initial state
        assert_eq!(ram8.output(&clock, [O, O, O]), zero);
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        // set one into register[address]
        ram8.input(&clock, one, [O, O, O], I); // register[0]: prev: 0, cur: 1
        assert_eq!(ram8.output(&clock, [O, O, O]), zero);
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tock (should happen nothing)
        ram8.input(&clock, one, [O, O, I], I); // register[0]: prev: 0, cur: 1
        assert_eq!(ram8.output(&clock, [O, O, O]), one); //
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tick
        ram8.input(&clock, one, [O, O, I], I); // register[0]: prev: 1, cur: I, register[1]: prev: 0, cur: 1
        assert_eq!(ram8.output(&clock, [O, O, O]), one);
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tock
        ram8.input(&clock, one, [O, O, I], I); // register[0]: prev: 1, cur: I, register[1]: prev: 0, cur: 1
        assert_eq!(ram8.output(&clock, [O, O, O]), one); // remained
        assert_eq!(ram8.output(&clock, [O, O, I]), one); // updated
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tick
        ram8.input(&clock, zero, [O, O, O], I); // register[0]をzeroに更新 register[0]: prev: 1, cur: 0, register[1]: prev: 1, cur: 1
        assert_eq!(ram8.output(&clock, [O, O, O]), one);
        assert_eq!(ram8.output(&clock, [O, O, I]), one);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tock (Tock時間のoutputにあまり興味はない)
        ram8.input(&clock, zero, [O, O, O], I); // register[0]: prev: 1, cur: 0, register[1]: prev: 1, cur: 1
        assert_eq!(ram8.output(&clock, [O, O, O]), zero); // curの値
        assert_eq!(ram8.output(&clock, [O, O, I]), one);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tick
        ram8.input(&clock, one, [O, O, I], I); // register[1]をoneに更新 register[1]: prev:0, cur:1
        assert_eq!(ram8.output(&clock, [O, O, O]), zero);
        assert_eq!(ram8.output(&clock, [O, O, I]), one);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tock
        assert_eq!(ram8.output(&clock, [O, O, O]), zero);
        assert_eq!(ram8.output(&clock, [O, O, I]), one);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next();
        // Tick
        assert_eq!(ram8.output(&clock, [O, O, O]), zero);
        assert_eq!(ram8.output(&clock, [O, O, I]), one);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);
    }

    #[test]
    fn for_ram8_2() {
        let zero = Word::new([O; 16]);
        let word1 = Word::new([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]);
        let mut ram8 = RAM8::new();
        let mut clock = Clock::new();

        // Tick
        ram8.input(&clock, word1, [O, O, O], I);

        clock.next(); // Tock
        clock.next(); // Tick
        ram8.input(&clock, word1, [I, O, I], I);
        assert_eq!(ram8.output(&clock, [O, O, O]), word1);
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), zero);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next(); // Tock
        clock.next(); // Tick
        ram8.input(&clock, word1, [I, I, I], O);
        assert_eq!(ram8.output(&clock, [O, O, O]), word1);
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), word1);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next(); // Tock
        clock.next(); // Tick
        ram8.input(&clock, zero, [O, O, O], I);
        assert_eq!(ram8.output(&clock, [O, O, O]), word1);
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), word1);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);

        clock.next(); // Tock
        clock.next(); // Tick
        ram8.input(&clock, zero, [O, O, O], O);
        assert_eq!(ram8.output(&clock, [O, O, O]), zero);
        assert_eq!(ram8.output(&clock, [O, O, I]), zero);
        assert_eq!(ram8.output(&clock, [O, I, O]), zero);
        assert_eq!(ram8.output(&clock, [O, I, I]), zero);
        assert_eq!(ram8.output(&clock, [I, O, O]), zero);
        assert_eq!(ram8.output(&clock, [I, O, I]), word1);
        assert_eq!(ram8.output(&clock, [I, I, O]), zero);
        assert_eq!(ram8.output(&clock, [I, I, I]), zero);
    }

    #[test]
    fn for_ram64() {
        let zero = Word::new([O; 16]);
        let mut clock = Clock::new();
        let mut ram64 = RAM64::new();
        let word1 = Word::new([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]);

        ram64.input(&clock, word1, [O, O, O, O, O, O], I);
        assert_eq!(ram64.output(&clock, [O, O, O, O, O, O]), zero);

        clock.next();
        clock.next();
        ram64.input(&clock, word1, [O, O, I, O, O, O], I);
        assert_eq!(ram64.output(&clock, [O, O, O, O, O, O]), word1);

        clock.next();
        clock.next();
        ram64.input(&clock, word1, [O, O, O, O, O, I], O);
        assert_eq!(ram64.output(&clock, [O, O, O, O, O, O]), word1);
        assert_eq!(ram64.output(&clock, [O, O, I, O, O, O]), word1);

        clock.next();
        clock.next();
        ram64.input(&clock, word1, [O, O, O, O, O, I], I);
        assert_eq!(ram64.output(&clock, [O, O, O, O, O, O]), word1);
        assert_eq!(ram64.output(&clock, [O, O, I, O, O, O]), word1);
        assert_eq!(ram64.output(&clock, [O, O, O, O, O, I]), zero);

        clock.next();
        clock.next();
        ram64.input(&clock, word1, [O, O, O, O, O, I], I);
        assert_eq!(ram64.output(&clock, [O, O, O, O, O, O]), word1);
        assert_eq!(ram64.output(&clock, [O, O, I, O, O, O]), word1);
        assert_eq!(ram64.output(&clock, [O, O, O, O, O, I]), word1);
    }
}
