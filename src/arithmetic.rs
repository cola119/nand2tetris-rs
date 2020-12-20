#![allow(dead_code)]
use crate::logic::bit::{I, O};
use crate::logic::*;

pub fn half_adder(a: bit, b: bit) -> [bit; 2] {
    [and(a, b), xor(a, b)]
}

pub fn full_adder(a: bit, b: bit, c: bit) -> [bit; 2] {
    let bc = half_adder(b, c);
    let a_bc1 = half_adder(a, bc[1]);
    [or(bc[0], a_bc1[0]), a_bc1[1]]
}

pub fn inc(a: bit) -> [bit; 2] {
    half_adder(a, I)
}

pub fn add16(a: Word, b: Word) -> Word {
    let added15 = half_adder(a[15], b[15]);
    let added14 = full_adder(a[14], b[14], added15[0]);
    let added13 = full_adder(a[13], b[13], added14[0]);
    let added12 = full_adder(a[12], b[12], added13[0]);
    let added11 = full_adder(a[11], b[11], added12[0]);
    let added10 = full_adder(a[10], b[10], added11[0]);
    let added9 = full_adder(a[9], b[9], added10[0]);
    let added8 = full_adder(a[8], b[8], added9[0]);
    let added7 = full_adder(a[7], b[7], added8[0]);
    let added6 = full_adder(a[6], b[6], added7[0]);
    let added5 = full_adder(a[5], b[5], added6[0]);
    let added4 = full_adder(a[4], b[4], added5[0]);
    let added3 = full_adder(a[3], b[3], added4[0]);
    let added2 = full_adder(a[2], b[2], added3[0]);
    let added1 = full_adder(a[1], b[1], added2[0]);
    let added0 = full_adder(a[0], b[0], added1[0]);
    Word::new([
        added0[1], added1[1], added2[1], added3[1], added4[1], added5[1], added6[1], added7[1],
        added8[1], added9[1], added10[1], added11[1], added12[1], added13[1], added14[1],
        added15[1],
    ])
}

pub fn inc16(a: Word) -> Word {
    add16(
        a,
        Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I]),
    )
}

#[cfg(test)]
mod tests {
    use super::{add16, full_adder, half_adder, inc, inc16};
    use crate::logic::bit::{I, O};
    use crate::Word;

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

    #[test]
    fn for_inc() {
        assert_eq!(inc(O), [O, I]);
        assert_eq!(inc(I), [I, O]);
    }

    #[test]
    fn for_inc16() {
        assert_eq!(
            inc16(Word::new([O; 16])),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I])
        );
        assert_eq!(
            inc16(Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I])),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, O])
        );
        assert_eq!(
            inc16(Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, O])),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, I, I])
        );
        assert_eq!(
            inc16(Word::new([I; 16])),
            Word::new([O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O])
        );
    }

    #[test]
    fn for_add16() {
        assert_eq!(
            add16(Word::new([I; 16]), Word::new([O; 16])),
            Word::new([I; 16])
        );
        assert_eq!(
            add16(
                Word::new([O, O, O, O, I, I, I, I, O, I, O, I, O, O, I, I]),
                Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])
            ),
            Word::new([O, I, I, I, O, O, I, I, I, I, I, O, I, I, O, I])
        );
        assert_eq!(
            add16(
                Word::new([I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I]),
                Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, I, O])
            ),
            Word::new([O, I, I, O, O, I, O, O, I, O, O, I, I, O, O, I])
        );
    }
}
