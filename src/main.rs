use crate::Bit::{I, O};
use std::ops::Index;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bit {
    O,
    I,
}

#[derive(Debug, PartialEq)]
pub struct Word([Bit; 16]);

impl Index<usize> for Word {
    type Output = Bit;
    fn index(&self, index: usize) -> &Self::Output {
        if index > 15 {
            panic!(format!("index {} is out of range.", index));
        }
        &self.0[index]
    }
}

pub fn nand(a: Bit, b: Bit) -> Bit {
    match a {
        O => match b {
            O => I,
            I => I,
        },
        I => match b {
            O => I,
            I => O,
        },
    }
}

pub fn not(a: Bit) -> Bit {
    nand(a, I)
}

pub fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

pub fn or(a: Bit, b: Bit) -> Bit {
    nand(not(a), not(b))
}

pub fn xor(a: Bit, b: Bit) -> Bit {
    and(or(a, b), nand(a, b))
}

pub fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    xor(and(a, not(sel)), and(b, sel))
}

pub fn dmux(input: Bit, sel: Bit) -> [Bit; 2] {
    [and(input, not(sel)), and(input, sel)]
}

pub fn not16(bits: Word) -> Word {
    Word([
        not(bits[0]),
        not(bits[1]),
        not(bits[2]),
        not(bits[3]),
        not(bits[4]),
        not(bits[5]),
        not(bits[6]),
        not(bits[7]),
        not(bits[8]),
        not(bits[9]),
        not(bits[10]),
        not(bits[11]),
        not(bits[12]),
        not(bits[13]),
        not(bits[14]),
        not(bits[15]),
    ])
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::Bit::{I, O};
    use super::{and, dmux, mux, nand, not, not16, or, xor, Word};

    #[test]
    fn for_nand() {
        assert_eq!(nand(O, O), I);
        assert_eq!(nand(I, O), I);
        assert_eq!(nand(O, I), I);
        assert_eq!(nand(I, I), O);
    }

    #[test]
    fn for_not() {
        assert_eq!(not(O), I);
        assert_eq!(not(I), O);
    }

    #[test]
    fn for_and() {
        assert_eq!(and(O, O), O);
        assert_eq!(and(I, O), O);
        assert_eq!(and(O, I), O);
        assert_eq!(and(I, I), I);
    }

    #[test]
    fn for_or() {
        assert_eq!(or(O, O), O);
        assert_eq!(or(I, O), I);
        assert_eq!(or(O, I), I);
        assert_eq!(or(I, I), I);
    }

    #[test]
    fn for_xor() {
        assert_eq!(xor(O, O), O);
        assert_eq!(xor(I, O), I);
        assert_eq!(xor(O, I), I);
        assert_eq!(xor(I, I), O);
    }

    #[test]
    fn for_mux() {
        assert_eq!(mux(O, O, O), O);
        assert_eq!(mux(O, I, O), O);
        assert_eq!(mux(I, O, O), I);
        assert_eq!(mux(I, I, O), I);
        assert_eq!(mux(O, O, I), O);
        assert_eq!(mux(O, I, I), I);
        assert_eq!(mux(I, O, I), O);
        assert_eq!(mux(I, I, I), I);
    }

    #[test]
    fn for_dmux() {
        assert_eq!(dmux(O, O), [O, O]);
        assert_eq!(dmux(O, I), [O, O]);
        assert_eq!(dmux(I, O), [I, O]);
        assert_eq!(dmux(I, I), [O, I]);
    }

    #[test]
    fn for_not16() {
        assert_eq!(not16(Word([O; 16])), Word([I; 16]));
        assert_eq!(not16(Word([I; 16])), Word([O; 16]));
        assert_eq!(
            not16(Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])),
            Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])
        );
        assert_eq!(
            not16(Word([I, I, I, O, I, O, I, O, I, O, I, O, I, I, I, O])),
            Word([O, O, O, I, O, I, O, I, O, I, O, I, O, O, O, I])
        );
    }
}
