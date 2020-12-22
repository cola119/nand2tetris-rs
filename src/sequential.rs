#![allow(dead_code)]
use crate::dff::ClockState::{Tick, Tock};
use crate::{
    dff::Clock,
    logic::{
        bit::{I, O},
        mux,
    },
};
use crate::{dff::Dff, logic::bit};

#[derive(Copy, Clone)]
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
                c.state = Tock;
                c
            }
            Tock => Clock::new(),
        };
        let out_t_1 = self.dff.output(&clock_t_1);
        // Save out(t) into DFF
        // if load(t-1) then out(t) = in(t-1)
        // else out(t) = out(t-1)
        self.dff.input(clock_t, mux(out_t_1, input, load))
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
        // Tock prev: I, cur: I
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
}
