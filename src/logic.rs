#![allow(dead_code)]
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

pub fn and16(a: Word, b: Word) -> Word {
    Word([
        and(a[0], b[0]),
        and(a[1], b[1]),
        and(a[2], b[2]),
        and(a[3], b[3]),
        and(a[4], b[4]),
        and(a[5], b[5]),
        and(a[6], b[6]),
        and(a[7], b[7]),
        and(a[8], b[8]),
        and(a[9], b[9]),
        and(a[10], b[10]),
        and(a[11], b[11]),
        and(a[12], b[12]),
        and(a[13], b[13]),
        and(a[14], b[14]),
        and(a[15], b[15]),
    ])
}

pub fn or16(a: Word, b: Word) -> Word {
    Word([
        or(a[0], b[0]),
        or(a[1], b[1]),
        or(a[2], b[2]),
        or(a[3], b[3]),
        or(a[4], b[4]),
        or(a[5], b[5]),
        or(a[6], b[6]),
        or(a[7], b[7]),
        or(a[8], b[8]),
        or(a[9], b[9]),
        or(a[10], b[10]),
        or(a[11], b[11]),
        or(a[12], b[12]),
        or(a[13], b[13]),
        or(a[14], b[14]),
        or(a[15], b[15]),
    ])
}

pub fn mux16(a: Word, b: Word, sel: Bit) -> Word {
    Word([
        mux(a[0], b[0], sel),
        mux(a[1], b[1], sel),
        mux(a[2], b[2], sel),
        mux(a[3], b[3], sel),
        mux(a[4], b[4], sel),
        mux(a[5], b[5], sel),
        mux(a[6], b[6], sel),
        mux(a[7], b[7], sel),
        mux(a[8], b[8], sel),
        mux(a[9], b[9], sel),
        mux(a[10], b[10], sel),
        mux(a[11], b[11], sel),
        mux(a[12], b[12], sel),
        mux(a[13], b[13], sel),
        mux(a[14], b[14], sel),
        mux(a[15], b[15], sel),
    ])
}

#[cfg(test)]
mod tests {
    use super::Bit::{I, O};
    use super::{and, and16, dmux, mux, mux16, nand, not, not16, or, or16, xor, Word};

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

    #[test]
    fn for_and16() {
        assert_eq!(and16(Word([O; 16]), Word([O; 16])), Word([O; 16]));
        assert_eq!(and16(Word([I; 16]), Word([O; 16])), Word([O; 16]));
        assert_eq!(and16(Word([O; 16]), Word([I; 16])), Word([O; 16]));
        assert_eq!(and16(Word([I; 16]), Word([I; 16])), Word([I; 16]));
        assert_eq!(
            and16(
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
            ),
            Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            and16(
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])
            ),
            Word([O; 16])
        );
    }

    #[test]
    fn for_or16() {
        assert_eq!(or16(Word([O; 16]), Word([O; 16])), Word([O; 16]));
        assert_eq!(or16(Word([I; 16]), Word([O; 16])), Word([I; 16]));
        assert_eq!(or16(Word([O; 16]), Word([I; 16])), Word([I; 16]));
        assert_eq!(or16(Word([I; 16]), Word([I; 16])), Word([I; 16]));
        assert_eq!(
            or16(
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
            ),
            Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            or16(
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])
            ),
            Word([I; 16])
        );
    }

    #[test]
    fn for_mux16() {
        assert_eq!(mux16(Word([O; 16]), Word([O; 16]), O), Word([O; 16]));
        assert_eq!(mux16(Word([O; 16]), Word([I; 16]), O), Word([O; 16]));
        assert_eq!(mux16(Word([I; 16]), Word([O; 16]), O), Word([I; 16]));
        assert_eq!(mux16(Word([I; 16]), Word([I; 16]), O), Word([I; 16]));
        assert_eq!(mux16(Word([O; 16]), Word([O; 16]), I), Word([O; 16]));
        assert_eq!(mux16(Word([I; 16]), Word([O; 16]), I), Word([O; 16]));
        assert_eq!(mux16(Word([O; 16]), Word([I; 16]), I), Word([I; 16]));
        assert_eq!(mux16(Word([I; 16]), Word([I; 16]), I), Word([I; 16]));
        assert_eq!(
            mux16(
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
                O
            ),
            Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
        assert_eq!(
            mux16(
                Word([I, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O]),
                Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I]),
                I
            ),
            Word([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])
        );
    }
}
