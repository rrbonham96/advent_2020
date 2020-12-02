//! Advent of Code utilities
use std::fs;
use std::io::{self, BufRead};

pub fn load_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}