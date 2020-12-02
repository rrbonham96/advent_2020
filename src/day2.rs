//! Day 2
use regex::Regex;

pub fn valid_passwords(lines: &Vec<String>) -> i32 {
    let mut valid_count = 0;
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    for line in lines.iter() {
        for cap in re.captures_iter(line) {
            let min: u32 = cap[1].parse().unwrap();
            let max: u32 = cap[2].parse().unwrap();
            let target: char = cap[3].chars().next().unwrap();
            let pw = cap[4].to_string();

            let char_count = pw.chars().filter(|&c| c == target).count();
            if char_count as u32 >= min && char_count as u32 <= max {
                valid_count += 1;
            }
        }
    }
    valid_count
}

pub fn actually_valid_passwords(lines: &Vec<String>) -> i32 {
    let mut valid_count = 0;
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    for line in lines.iter() {
        for cap in re.captures_iter(line) {
            let min = cap[1].parse::<usize>().unwrap() - 1;
            let max = cap[2].parse::<usize>().unwrap() - 1;
            let target: char = cap[3].chars().next().unwrap();
            let pw: Vec<char> = cap[4].chars().collect();

            if (pw[min] == target || pw[max] == target) && pw[min] != pw[max] {
                valid_count += 1;
            }
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn solve_p1() {
        let input = util::load_file("inputs/day2_input.txt");
        let lines = input.split('\n').map(|s| s.to_string()).collect();
        let valid = valid_passwords(&lines);
        println!("Valid passwords: {}", valid);
    }

    #[test]
    fn solve_p2() {
        let input = util::load_file("inputs/day2_input.txt");
        let lines = input.split('\n').map(|s| s.to_string()).collect();
        let valid = actually_valid_passwords(&lines);
        println!("Valid passwords: {}", valid);
    }
}