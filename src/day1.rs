//! Day one
use crate::util;

type BitField = Vec<u32>;

fn set_bit(b: &mut BitField, pos: u32) {
    let offset = pos / 32;
    let bit = pos % 32;
    b[offset as usize] |= 1 << bit;
}

fn get_bit(b: &BitField, pos: u32) -> bool {
    let offset = pos / 32;
    let bit = pos % 32;
    b[offset as usize] & (1 << bit) != 0
}

/// Complements finds two numbers in a list of numbers which sum to the target value
pub fn complements(numbers: &[u32], target: u32) -> Option<(u32, u32)> {
    let mut visited: BitField = vec![0u32; target as usize / 32 + 1];
    for &number in numbers.iter().filter(|&n| *n <= target) {
        let diff = target - number;
        if get_bit(&visited, diff) {
            return Some((number, diff));
        } else {
            set_bit(&mut visited, number);
        }
    }
    None
}

/// Three Complements finds three numbers that sum to the target value
pub fn three_complements(numbers: &[u32], target: u32) -> Option<(u32, u32, u32)> {
    for &number in numbers.iter() {
        let diff = target - number;
        match complements(&numbers, diff) {
            None => continue,
            Some((x, y)) => return Some((x, y, number)),
        }
    }
    None
}

pub fn list_of_nums(filename: &str) -> Vec<u32> {
    util::load_file(filename)
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_p1() {
        let list = list_of_nums("inputs/day1_input.txt");
        match complements(&list, 2020){
            None => panic!("No complements found!"),
            Some((x, y)) => println!("Solution: {}", x * y),
        }
    }

    #[test]
    fn solve_p2() {
        let list = list_of_nums("inputs/day1_input.txt");
        match three_complements(&list, 2020){
            None => panic!("No complements found!"),
            Some((x, y, z)) => println!("Solution: {}", x * y * z),
        }
    }
}