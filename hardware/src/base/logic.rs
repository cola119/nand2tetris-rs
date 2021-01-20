#![allow(dead_code, non_camel_case_types)]

use crate::base::logic::bit::{I, O};
use core::panic;
use std::{
    fmt::{self, Formatter},
    ops::Index,
    write,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum bit {
    O,
    I,
}
impl fmt::Display for bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = match self {
            O => "0".to_string(),
            I => "1".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Word([bit; 16]);

impl From<&str> for Word {
    // TODO from(number)

    fn from(str: &str) -> Self {
        let char_bits: Vec<char> = str.chars().collect();
        if char_bits.len() != 16 {
            panic!(&format!("couldn't parse {:?}", str));
        }
        let mut bits = [O; 16];
        for i in 0..16 {
            bits[i] = match char_bits[i].to_digit(10) {
                Some(0) => O,
                Some(1) => I,
                _ => panic!(&format!("unknown number {}", char_bits[i])),
            }
        }
        Word::new(bits)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = self
            .0
            .iter()
            .map(|bit| -> String {
                match bit {
                    O => "0".to_string(),
                    I => "1".to_string(),
                }
            })
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", str)
    }
}

impl Word {
    pub fn new(bits: [bit; 16]) -> Self {
        Self(bits)
    }
}

impl Index<usize> for Word {
    type Output = bit;
    fn index(&self, index: usize) -> &Self::Output {
        if index > 15 {
            panic!(&format!("index {} is out of range.", index));
        }
        &self.0[index]
    }
}

pub fn nand(a: bit, b: bit) -> bit {
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

pub fn not(a: bit) -> bit {
    nand(a, I)
}

pub fn and(a: bit, b: bit) -> bit {
    not(nand(a, b))
}

pub fn or(a: bit, b: bit) -> bit {
    nand(not(a), not(b))
}

pub fn xor(a: bit, b: bit) -> bit {
    and(or(a, b), nand(a, b))
}

pub fn mux(a: bit, b: bit, sel: bit) -> bit {
    xor(and(a, not(sel)), and(b, sel))
}

pub fn dmux(input: bit, sel: bit) -> [bit; 2] {
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

pub fn mux16(a: Word, b: Word, sel: bit) -> Word {
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

pub fn or8way(a: [bit; 8]) -> bit {
    or(
        or(or(a[0], a[7]), or(a[1], a[6])),
        or(or(a[2], a[5]), or(a[3], a[4])),
    )
}

pub fn mux2(a: bit, b: bit, c: bit, d: bit, sel: [bit; 2]) -> bit {
    mux(mux(a, b, sel[1]), mux(c, d, sel[1]), sel[0])
}

pub fn mux3(a: bit, b: bit, c: bit, d: bit, e: bit, f: bit, g: bit, h: bit, sel: [bit; 3]) -> bit {
    mux2(
        mux(a, b, sel[2]),
        mux(c, d, sel[2]),
        mux(e, f, sel[2]),
        mux(g, h, sel[2]),
        [sel[0], sel[1]],
    )
}

pub fn mux4way16(a: Word, b: Word, c: Word, d: Word, sel: [bit; 2]) -> Word {
    Word::new([
        mux2(a[0], b[0], c[0], d[0], sel),
        mux2(a[1], b[1], c[1], d[1], sel),
        mux2(a[2], b[2], c[2], d[2], sel),
        mux2(a[3], b[3], c[3], d[3], sel),
        mux2(a[4], b[4], c[4], d[4], sel),
        mux2(a[5], b[5], c[5], d[5], sel),
        mux2(a[6], b[6], c[6], d[6], sel),
        mux2(a[7], b[7], c[7], d[7], sel),
        mux2(a[8], b[8], c[8], d[8], sel),
        mux2(a[9], b[9], c[9], d[9], sel),
        mux2(a[10], b[10], c[10], d[10], sel),
        mux2(a[11], b[11], c[11], d[11], sel),
        mux2(a[12], b[12], c[12], d[12], sel),
        mux2(a[13], b[13], c[13], d[13], sel),
        mux2(a[14], b[14], c[14], d[14], sel),
        mux2(a[15], b[15], c[15], d[15], sel),
    ])
}

pub fn mux8way16(
    a: Word,
    b: Word,
    c: Word,
    d: Word,
    e: Word,
    f: Word,
    g: Word,
    h: Word,
    sel: [bit; 3],
) -> Word {
    Word::new([
        mux3(a[0], b[0], c[0], d[0], e[0], f[0], g[0], h[0], sel),
        mux3(a[1], b[1], c[1], d[1], e[1], f[1], g[1], h[1], sel),
        mux3(a[2], b[2], c[2], d[2], e[2], f[2], g[2], h[2], sel),
        mux3(a[3], b[3], c[3], d[3], e[3], f[3], g[3], h[3], sel),
        mux3(a[4], b[4], c[4], d[4], e[4], f[4], g[4], h[4], sel),
        mux3(a[5], b[5], c[5], d[5], e[5], f[5], g[5], h[5], sel),
        mux3(a[6], b[6], c[6], d[6], e[6], f[6], g[6], h[6], sel),
        mux3(a[7], b[7], c[7], d[7], e[7], f[7], g[7], h[7], sel),
        mux3(a[8], b[8], c[8], d[8], e[8], f[8], g[8], h[8], sel),
        mux3(a[9], b[9], c[9], d[9], e[9], f[9], g[9], h[9], sel),
        mux3(a[10], b[10], c[10], d[10], e[10], f[10], g[10], h[10], sel),
        mux3(a[11], b[11], c[11], d[11], e[11], f[11], g[11], h[11], sel),
        mux3(a[12], b[12], c[12], d[12], e[12], f[12], g[12], h[12], sel),
        mux3(a[13], b[13], c[13], d[13], e[13], f[13], g[13], h[13], sel),
        mux3(a[14], b[14], c[14], d[14], e[14], f[14], g[14], h[14], sel),
        mux3(a[15], b[15], c[15], d[15], e[15], f[15], g[15], h[15], sel),
    ])
}

pub fn dmux4way(input: bit, sel: [bit; 2]) -> [bit; 4] {
    [
        and(and(not(xor(sel[0], O)), not(xor(sel[1], O))), input),
        and(and(not(xor(sel[0], O)), not(xor(sel[1], I))), input),
        and(and(not(xor(sel[0], I)), not(xor(sel[1], O))), input),
        and(and(not(xor(sel[0], I)), not(xor(sel[1], I))), input),
    ]
}

pub fn dmux8way(input: bit, sel: [bit; 3]) -> [bit; 8] {
    let nxor = |a: bit, b: bit| -> bit { not(xor(a, b)) };
    [
        and(
            and(and(nxor(sel[0], O), nxor(sel[1], O)), nxor(sel[2], O)),
            input,
        ),
        and(
            and(and(nxor(sel[0], O), nxor(sel[1], O)), nxor(sel[2], I)),
            input,
        ),
        and(
            and(and(nxor(sel[0], O), nxor(sel[1], I)), nxor(sel[2], O)),
            input,
        ),
        and(
            and(and(nxor(sel[0], O), nxor(sel[1], I)), nxor(sel[2], I)),
            input,
        ),
        and(
            and(and(nxor(sel[0], I), nxor(sel[1], O)), nxor(sel[2], O)),
            input,
        ),
        and(
            and(and(nxor(sel[0], I), nxor(sel[1], O)), nxor(sel[2], I)),
            input,
        ),
        and(
            and(and(nxor(sel[0], I), nxor(sel[1], I)), nxor(sel[2], O)),
            input,
        ),
        and(
            and(and(nxor(sel[0], I), nxor(sel[1], I)), nxor(sel[2], I)),
            input,
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::{
        and, and16, dmux, dmux4way, dmux8way, mux, mux16, nand, not, not16, or, or16, xor, Word,
    };
    use super::{
        bit::{I, O},
        or8way,
    };
    use crate::base::logic::mux2;
    use crate::base::logic::mux3;
    use crate::base::logic::mux4way16;
    use crate::base::logic::mux8way16;

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

    #[test]
    fn for_or8way() {
        assert_eq!(or8way([O; 8]), O);
        assert_eq!(or8way([I; 8]), I);
        assert_eq!(or8way([O, O, O, O, O, O, O, I]), I);
        assert_eq!(or8way([O, O, O, O, O, O, I, I]), I);
        assert_eq!(or8way([O, O, I, O, O, O, I, I]), I);
        assert_eq!(or8way([O, O, O, I, O, O, O, O]), I);
    }

    #[test]
    fn for_mux2() {
        assert_eq!(mux2(I, O, O, O, [O, O]), I);
        assert_eq!(mux2(O, I, O, O, [O, I]), I);
        assert_eq!(mux2(O, O, I, O, [I, O]), I);
        assert_eq!(mux2(O, O, O, I, [I, I]), I);
    }

    #[test]
    fn for_mux3() {
        assert_eq!(mux3(I, O, O, O, O, O, O, O, [O, O, O]), I);
        assert_eq!(mux3(O, I, O, O, O, O, O, O, [O, O, I]), I);
        assert_eq!(mux3(O, O, I, O, O, O, O, O, [O, I, O]), I);
        assert_eq!(mux3(O, O, O, I, O, O, O, O, [O, I, I]), I);
        assert_eq!(mux3(O, O, O, O, I, O, O, O, [I, O, O]), I);
        assert_eq!(mux3(O, O, O, O, O, I, O, O, [I, O, I]), I);
        assert_eq!(mux3(O, O, O, O, O, O, I, O, [I, I, O]), I);
        assert_eq!(mux3(O, O, O, O, O, O, O, I, [I, I, I]), I);
    }

    #[test]
    fn for_mux4way16() {
        let zero = Word::new([O; 16]);
        let one = Word::new([I; 16]);
        assert_eq!(mux4way16(one, zero, zero, zero, [O, O]), one);
        assert_eq!(mux4way16(zero, one, zero, zero, [O, I]), one);
        assert_eq!(mux4way16(zero, zero, one, zero, [I, O]), one);
        assert_eq!(mux4way16(zero, zero, zero, one, [I, I]), one);
    }

    #[test]
    fn for_mux8way16() {
        let zero = Word::new([O; 16]);
        let one = Word::new([I; 16]);
        assert_eq!(
            mux8way16(one, zero, zero, zero, zero, zero, zero, zero, [O, O, O]),
            one
        );
        assert_eq!(
            mux8way16(zero, one, zero, zero, zero, zero, zero, zero, [O, O, I]),
            one
        );
        assert_eq!(
            mux8way16(zero, zero, one, zero, zero, zero, zero, zero, [O, I, O]),
            one
        );
        assert_eq!(
            mux8way16(zero, zero, zero, one, zero, zero, zero, zero, [O, I, I]),
            one
        );
        assert_eq!(
            mux8way16(zero, zero, zero, zero, one, zero, zero, zero, [I, O, O]),
            one
        );
        assert_eq!(
            mux8way16(zero, zero, zero, zero, zero, one, zero, zero, [I, O, I]),
            one
        );
        assert_eq!(
            mux8way16(zero, zero, zero, zero, zero, zero, one, zero, [I, I, O]),
            one
        );
        assert_eq!(
            mux8way16(zero, zero, zero, zero, zero, zero, zero, one, [I, I, I]),
            one
        );
    }

    #[test]
    fn for_dmux4way() {
        assert_eq!(dmux4way(I, [O, O]), [I, O, O, O]);
        assert_eq!(dmux4way(I, [O, I]), [O, I, O, O]);
        assert_eq!(dmux4way(I, [I, O]), [O, O, I, O]);
        assert_eq!(dmux4way(I, [I, I]), [O, O, O, I]);
    }

    #[test]
    fn for_dmux8way() {
        assert_eq!(dmux8way(I, [O, O, O]), [I, O, O, O, O, O, O, O]);
        assert_eq!(dmux8way(I, [O, O, I]), [O, I, O, O, O, O, O, O]);
        assert_eq!(dmux8way(I, [O, I, O]), [O, O, I, O, O, O, O, O]);
        assert_eq!(dmux8way(I, [O, I, I]), [O, O, O, I, O, O, O, O]);
        assert_eq!(dmux8way(I, [I, O, O]), [O, O, O, O, I, O, O, O]);
        assert_eq!(dmux8way(I, [I, O, I]), [O, O, O, O, O, I, O, O]);
        assert_eq!(dmux8way(I, [I, I, O]), [O, O, O, O, O, O, I, O]);
        assert_eq!(dmux8way(I, [I, I, I]), [O, O, O, O, O, O, O, I]);
    }

    #[test]
    fn for_word_from() {
        assert_eq!(Word::from("0000000000000000"), Word::new([O; 16]));
        assert_eq!(Word::from("1111111111111111"), Word::new([I; 16]));
        assert_eq!(
            Word::from("0101010101010101"),
            Word::new([O, I, O, I, O, I, O, I, O, I, O, I, O, I, O, I])
        );
        assert_eq!(
            Word::from("0010101010101010"),
            Word::new([O, O, I, O, I, O, I, O, I, O, I, O, I, O, I, O])
        );
    }

    #[test]
    #[should_panic]
    fn for_word_from2() {
        Word::from("11");
    }
    #[test]
    #[should_panic]
    fn for_word_from3() {
        Word::from("000000000000000a");
    }
}
