use crate::Bit::{O, I};

#[derive(Debug, PartialEq)]
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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{nand};
    use super::Bit::{O, I};

    #[test]
    fn for_nand() {
        assert_eq!(nand(O, O), I);
        assert_eq!(nand(I, O), I);
        assert_eq!(nand(O, I), I);
        assert_eq!(nand(I, I), O);
    }
}