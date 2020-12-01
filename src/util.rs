//! Advent of Code utilities
use std::fs;

pub fn load_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}