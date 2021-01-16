#![allow(dead_code)]

use crate::base::logic::bit;
use crate::base::logic::bit::{I, O};

pub fn read_stdin<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn str_to_binary(str: &str) -> Vec<bit> {
    if str == "" {
        panic!(format!("Could't convert an empty string"))
    }

    let char_bits: Vec<char> = str.chars().collect();
    let mut bits = Vec::new();

    for i in 0..char_bits.len() {
        let bit = match char_bits[i].to_digit(10) {
            Some(0) => O,
            Some(1) => I,
            _ => panic!("Unknown number. String should consist of characters neither 0 or 1"),
        };
        bits.push(bit);
    }

    return bits;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_str_to_binary() {
        assert_eq!(str_to_binary("1"), [I]);
        assert_eq!(str_to_binary("0"), [O]);
        assert_eq!(str_to_binary("000"), [O, O, O]);
        assert_eq!(str_to_binary("0101"), [O, I, O, I]);
    }

    #[test]
    #[should_panic]
    fn for_str_to_binary_2() {
        str_to_binary("");
    }

    #[test]
    #[should_panic]
    fn for_str_to_binary_3() {
        str_to_binary("a");
    }
}
