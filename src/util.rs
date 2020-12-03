//! Advent of Code utilities
use std::fs;
use std::io::{self, BufRead};

pub fn load_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let lines = load_file(filename);
    lines.split('\n').map(|s| s.to_string()).collect()
}