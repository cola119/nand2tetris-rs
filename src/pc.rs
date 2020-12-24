#![allow(dead_code)]
use crate::add16;
use crate::inc16;
use crate::{bit::I, logic::mux16};
use crate::{
    bit::O,
    logic::{not, or},
};
use crate::{
    dff::Clock,
    dff::ClockState::{Tick, Tock},
    logic::{bit, Word},
    register::Register,
};

#[derive(Debug, Clone, Copy)]
pub struct PC {
    register: Register,
}

impl PC {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }
    pub fn input(&mut self, clock_t: &Clock, input: Word, incr: bit, load: bit, reset: bit) {
        let clock_t_1 = match clock_t.state {
            Tick => {
                let mut c = Clock::new();
                c.next();
                c
            }
            Tock => Clock::new(),
        };
        let out_t_1 = self.output(&clock_t_1);
        let zero = Word::new([O; 16]);

        let input_tmp = mux16(
            mux16(mux16(out_t_1, inc16(out_t_1), incr), input, load),
            zero,
            reset,
        );
        self.register.input(clock_t, input_tmp, I);
    }
    pub fn output(self, clock_t: &Clock) -> Word {
        self.register.output(clock_t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_pc() {
        let mut pc = PC::new();
        let mut clock = Clock::new();
        let zero = Word::new([O; 16]);

        pc.input(&clock, zero, O, I, O);
        assert_eq!(pc.output(&clock), zero);

        clock.next();
        clock.next();
        pc.input(&clock, zero, I, O, O);
        assert_eq!(pc.output(&clock), zero);

        clock.next();
        clock.next();
        pc.input(&clock, zero, I, O, O);
        assert_eq!(
            pc.output(&clock),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I])
        );

        clock.next();
        clock.next();
        pc.input(&clock, zero, I, O, O);
        assert_eq!(
            pc.output(&clock),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, O])
        );

        clock.next();
        clock.next();
        pc.input(&clock, zero, O, O, I);
        assert_eq!(
            pc.output(&clock),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I])
        );

        clock.next();
        clock.next();
        let word = Word::new([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]);
        pc.input(&clock, word, O, I, O);
        assert_eq!(
            pc.output(&clock),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );

        clock.next();
        clock.next();
        pc.input(&clock, zero, I, O, O);
        assert_eq!(pc.output(&clock), word);

        clock.next();
        clock.next();
        pc.input(&clock, zero, I, O, O);
        assert_eq!(pc.output(&clock), inc16(word));
    }
}
