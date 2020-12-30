#![allow(dead_code)]
use crate::dff::ClockState::{Tick, Tock};
use crate::logic::bit;
use crate::logic::bit::{I, O};

#[derive(Debug, PartialEq)]
pub enum ClockState {
    Tick,
    Tock,
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    pub state: ClockState,
}

impl Clock {
    pub fn new() -> Self {
        Clock { state: Tick }
    }
    pub fn next(&mut self) {
        self.state = match self.state {
            Tick => Tock,
            Tock => Tick,
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dff {
    prev: bit,
    current: bit,
}

impl Dff {
    pub fn new() -> Self {
        Self {
            prev: O,
            current: O,
        }
    }
    pub fn input(&mut self, clock: &Clock, a: bit) {
        if clock.state == Tick {
            self.prev = self.current;
            self.current = a;
        }
    }
    pub fn output(&self, clock: &Clock) -> bit {
        match clock.state {
            Tick => self.prev,
            Tock => self.current,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_clock() {
        let mut clock = Clock::new();
        assert_eq!(clock.state, Tick);
        clock.next();
        assert_eq!(clock.state, Tock);
        clock.next();
        assert_eq!(clock.state, Tick);
        clock.next();
        clock.next();
        assert_eq!(clock.state, Tick);
    }

    #[test]
    fn for_dff() {
        let mut clock = Clock::new();
        // prev: 0, current: 0
        let mut dff = Dff::new();

        // tick: currentに入力値が入り、prev(i.e. t-1の値)が帰る
        // prev: 0, current: 1
        dff.input(&clock, I);
        assert_eq!(dff.output(&clock), O);
        clock.next();

        // tock: 何も起こらない。current(i.e. tの値)が帰る
        // prev: 0, current: 1
        dff.input(&clock, O);
        assert_eq!(dff.output(&clock), I);
        clock.next();

        // tick
        // prev: 1, current: 0
        dff.input(&clock, O);
        assert_eq!(dff.output(&clock), I);
        clock.next();

        // tock
        // prev: 1, current: 0
        dff.input(&clock, I);
        assert_eq!(dff.output(&clock), O);
        clock.next();

        // tick
        // prev: 1, current: 0
        assert_eq!(dff.output(&clock), I);
    }
}
