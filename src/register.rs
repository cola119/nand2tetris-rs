#![allow(dead_code)]
use crate::dff::ClockState::{Tick, Tock};
use crate::Word;
use crate::{
    dff::Clock,
    logic::{
        bit::{I, O},
        mux,
    },
};
use crate::{dff::Dff, logic::bit};

#[derive(Debug, Copy, Clone)]
pub struct Bit {
    dff: Dff,
}

impl Bit {
    pub fn new() -> Self {
        Self { dff: Dff::new() }
    }
    // clock(t), a(t-1), load(t-1)
    pub fn input(&mut self, clock_t: &Clock, input: bit, load: bit) {
        let clock_t_1 = match clock_t.state {
            Tick => {
                let mut c = Clock::new();
                c.next();
                c
            }
            Tock => Clock::new(),
        };
        let out_t_1 = self.output(&clock_t_1);
        // Save out(t) into DFF
        // if load(t-1) then out(t) = in(t-1)
        // else out(t) = out(t-1)
        self.dff.input(clock_t, mux(out_t_1, input, load));
    }
    pub fn output(self, clock: &Clock) -> bit {
        // Get out(t) from DFF
        self.dff.output(clock)
    }
    pub fn run(&mut self, clock_t: &Clock, input: bit, load: bit) -> bit {
        self.input(clock_t, input, load);
        self.output(clock_t)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Register {
    bits: [Bit; 16],
}

impl Register {
    pub fn new() -> Self {
        Self {
            bits: [Bit::new(); 16],
        }
    }
    pub fn input(&mut self, clock_t: &Clock, input: Word, load: bit) {
        self.bits[0].input(clock_t, input[0], load);
        self.bits[1].input(clock_t, input[1], load);
        self.bits[2].input(clock_t, input[2], load);
        self.bits[3].input(clock_t, input[3], load);
        self.bits[4].input(clock_t, input[4], load);
        self.bits[5].input(clock_t, input[5], load);
        self.bits[6].input(clock_t, input[6], load);
        self.bits[7].input(clock_t, input[7], load);
        self.bits[8].input(clock_t, input[8], load);
        self.bits[9].input(clock_t, input[9], load);
        self.bits[10].input(clock_t, input[10], load);
        self.bits[11].input(clock_t, input[11], load);
        self.bits[12].input(clock_t, input[12], load);
        self.bits[13].input(clock_t, input[13], load);
        self.bits[14].input(clock_t, input[14], load);
        self.bits[15].input(clock_t, input[15], load);
    }
    pub fn output(self, clock: &Clock) -> Word {
        Word::new([
            self.bits[0].output(clock),
            self.bits[1].output(clock),
            self.bits[2].output(clock),
            self.bits[3].output(clock),
            self.bits[4].output(clock),
            self.bits[5].output(clock),
            self.bits[6].output(clock),
            self.bits[7].output(clock),
            self.bits[8].output(clock),
            self.bits[9].output(clock),
            self.bits[10].output(clock),
            self.bits[11].output(clock),
            self.bits[12].output(clock),
            self.bits[13].output(clock),
            self.bits[14].output(clock),
            self.bits[15].output(clock),
        ])
    }
    pub fn run(&mut self, clock_t: &Clock, input: Word, load: bit) -> Word {
        self.input(clock_t, input, load);
        self.output(clock_t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_bit() {
        let mut clock = Clock::new();
        let mut bit = Bit::new();

        bit.input(&clock, I, I);
        // Tick prev: O, cur: I
        assert_eq!(bit.output(&clock), O);

        clock.next();
        bit.input(&clock, O, O);
        // Tock prev: I, cur: I  load=0の時、cur=prevとなるので古い値がが引き継がれてゆく。保存
        assert_eq!(bit.output(&clock), I);

        clock.next();
        bit.input(&clock, O, O);
        // Tick prev: I, cur: I
        assert_eq!(bit.output(&clock), I);

        clock.next();
        bit.input(&clock, O, O);
        // Tock prev: I, cur: I
        assert_eq!(bit.output(&clock), I);

        clock.next();
        bit.input(&clock, O, I);
        // Tick prev: I, cur: O
        assert_eq!(bit.output(&clock), I);

        clock.next();
        // Tock prev: O, cur: O
        bit.input(&clock, O, O);
        assert_eq!(bit.output(&clock), O);
    }

    #[test]
    fn for_bit2() {
        let mut clock = Clock::new();
        let mut bit = Bit::new();

        let out1 = bit.run(&clock, I, I);
        // Tick prev: O, cur: I
        assert_eq!(out1, O);

        clock.next();
        let out2 = bit.run(&clock, O, O);
        // Tock prev: I, cur: I
        assert_eq!(out2, I);

        clock.next();
        let out3 = bit.run(&clock, O, O);
        // Tick prev: I, cur: I
        assert_eq!(out3, I);

        clock.next();
        let out4 = bit.run(&clock, O, O);
        // Tock prev: I, cur: I
        assert_eq!(out4, I);

        clock.next();
        let out5 = bit.run(&clock, O, I);
        // Tick prev: I, cur: O
        assert_eq!(out5, I);

        clock.next();
        // Tock prev: O, cur: O
        let out6 = bit.run(&clock, O, O);
        assert_eq!(out6, O);
    }

    #[test]
    fn for_register() {
        let zero = Word::new([O; 16]);
        let one = Word::new([I; 16]);
        let mut clock = Clock::new();
        let mut register = Register::new();

        assert_eq!(register.output(&clock), zero);

        register.input(&clock, one, I);
        assert_eq!(register.output(&clock), zero);

        clock.next();
        register.input(&clock, one, O);
        assert_eq!(register.output(&clock), one);

        clock.next();
        register.input(&clock, one, O);
        assert_eq!(register.output(&clock), one);

        clock.next();
        register.input(&clock, zero, O);
        assert_eq!(register.output(&clock), one);

        clock.next();
        register.input(&clock, zero, I);
        assert_eq!(register.output(&clock), one);

        clock.next();
        register.input(&clock, zero, O);
        assert_eq!(register.output(&clock), zero);
    }

    #[test]
    fn for_register2() {
        let something = Word::new([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]);
        let zero = Word::new([O; 16]);
        let mut clock = Clock::new();
        let mut register = Register::new();

        assert_eq!(register.run(&clock, something, I), zero);

        clock.next();
        assert_eq!(register.run(&clock, something, O), something);
        clock.next();
        assert_eq!(register.run(&clock, something, O), something);
        clock.next();
        assert_eq!(register.run(&clock, zero, O), something);
        clock.next();
        assert_eq!(register.run(&clock, zero, I), something);
        clock.next();
        assert_eq!(register.run(&clock, zero, O), zero);
    }
}
