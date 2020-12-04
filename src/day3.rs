//! Day 3

pub fn multi_tree_count(map: &Vec<u32>, slopes: &Vec<(usize, usize)>) -> u64 {
    slopes.iter()
        .map(|(right, down)| fast_tree_count(map, *right, *down))
        .product::<u64>()
}

pub fn fast_tree_count(map: &Vec<u32>, right: usize, down: usize) -> u64 {
    map.iter()
        .enumerate()
        .step_by(down)
        .skip(down)
        .filter(|(i, b)| {
            (*b & 1 << ((i * right / down) % 31)) != 0
        })
        .count() as u64
}

pub fn input_to_bitfields(input: &Vec<String>) -> Vec<u32> {
    input.iter()
        .map(|s| {
            s.chars()
            .enumerate()
            .fold(0u32, |acc, (i, c)| acc | if c == '#' { 1u32 << i } else { 0u32 })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn solve_p1() {
        let input = util::read_lines("inputs/day3_input.txt");
        let char_matrix = input_to_bitfields(&input);
        let count = fast_tree_count(&char_matrix, 3, 1);
        println!("{}", count);
    }

    #[test]
    fn solve_p2() {
        let input = util::read_lines("inputs/day3_input.txt");
        let char_matrix = input_to_bitfields(&input);
        let slopes: Vec<(usize, usize)> = vec![
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2),
        ];
        let count = multi_tree_count(&char_matrix, &slopes);
        println!("{}", count);
    }
}