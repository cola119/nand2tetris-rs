use crate::Bit::{I, O};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Bit {
    O,
    I,
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::{dmux, mux};

    use super::Bit::{I, O};
    use super::{and, nand, not, or, xor};

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
}
