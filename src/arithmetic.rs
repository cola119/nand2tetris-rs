#![allow(dead_code)]
use crate::logic::Bit::{I, O};
use crate::logic::*;

pub fn half_adder(a: Bit, b: Bit) -> [Bit; 2] {
    [and(a, b), xor(a, b)]
}

pub fn full_adder(a: Bit, b: Bit, c: Bit) -> [Bit; 2] {
    let bc = half_adder(b, c);
    let a_bc1 = half_adder(a, bc[1]);
    [or(bc[0], a_bc1[0]), a_bc1[1]]
}

#[cfg(test)]
mod tests {
    use super::{full_adder, half_adder};
    use crate::logic::Bit::{I, O};

    #[test]
    fn for_half_adder() {
        assert_eq!(half_adder(O, O), [O, O]);
        assert_eq!(half_adder(O, I), [O, I]);
        assert_eq!(half_adder(I, O), [O, I]);
        assert_eq!(half_adder(I, I), [I, O]);
    }

    #[test]
    fn for_full_adder() {
        assert_eq!(full_adder(O, O, O), [O, O]);
        assert_eq!(full_adder(O, O, I), [O, I]);
        assert_eq!(full_adder(O, I, O), [O, I]);
        assert_eq!(full_adder(O, I, I), [I, O]);
        assert_eq!(full_adder(I, O, O), [O, I]);
        assert_eq!(full_adder(I, O, I), [I, O]);
        assert_eq!(full_adder(I, I, O), [I, O]);
        assert_eq!(full_adder(I, I, I), [I, I]);
    }
}
