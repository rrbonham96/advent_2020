//! Day one
use std::collections::HashMap;

/// Complements finds two numbers in a list of numbers which sum to the target value
pub fn complements(numbers: &[u32], target: u32) -> Option<(u32, u32)> {
    let mut visited: HashMap<u32, bool> = HashMap::new();
    for &number in numbers.iter() {
        if number > target {
            continue;
        }

        let diff = target - number;
        match visited.get(&diff) {
            None => { visited.insert(number, true); }
            Some(_) => { return Some((number, diff)); }
        }
    }
    None
}

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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn solve_p1() {
        let list_string = util::load_file("inputs/day1_input.txt");
        let list: Vec<u32> = list_string
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        match complements(&list, 2020){
            None => panic!("No complements found!"),
            Some((x, y)) => println!("Solution: {}", x * y),
        }
    }

    #[test]
    fn solve_p2() {
        let list_string = util::load_file("inputs/day1_input.txt");
        let list: Vec<u32> = list_string
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        match three_complements(&list, 2020){
            None => panic!("No complements found!"),
            Some((x, y, z)) => println!("Solution: {}", x * y * z),
        }
    }
}