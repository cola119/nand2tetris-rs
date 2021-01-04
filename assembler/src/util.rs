#![allow(dead_code)]

use std::{fs::File, io::BufRead, io::BufReader};
pub fn read_stdin<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

pub fn read_file_contents(filename: &str) -> String {
    let file = File::open(filename).expect(&format!("Failed to open {}", filename));
    BufReader::new(file)
        .lines()
        .map(|line| -> String { line.unwrap().to_string() })
        .collect::<Vec<String>>()
        .join("\n")
}
