//! Day 3

pub fn tree_count(map: &Vec<Vec<char>>, right: usize, down: usize) -> u64 {
    let mut tree_count = 0;
    let width = map[0].len();
    for (i, row) in map.iter().enumerate() {
        if i % down == 0 {
            if row[(right * i / down) % width] == '#' {
                tree_count += 1;
            }
        }
    }
    tree_count
}

pub fn multi_tree_count(map: &Vec<Vec<char>>, slopes: &Vec<(usize, usize)>) -> u64 {
    slopes.iter()
        .map(|(right, down)| tree_count(map, *right, *down))
        .product::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn solve_p1() {
        let input = util::read_lines("inputs/day3_input.txt");
        let mut char_matrix = Vec::new();
        for s in input {
            char_matrix.push(s.chars().collect::<Vec<char>>());
        }
        let count = tree_count(&char_matrix, 3, 1);
        println!("{}", count);
    }

    #[test]
    fn solve_p2() {
        let input = util::read_lines("inputs/day3_input.txt");
        let mut char_matrix = Vec::new();
        for s in input {
            char_matrix.push(s.chars().collect::<Vec<char>>());
        }
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