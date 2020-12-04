use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_2020::day3::*;
use advent_2020::util;

pub fn tree_count_bench(c: &mut Criterion) {
    let input = util::read_lines("inputs/day3_input.txt");
    let char_matrix = input_to_bitfields(&input);
    c.bench_function("Day 3 - Part 1",
     |b| b.iter(|| {
            fast_tree_count(black_box(&char_matrix),
             black_box(3),
              black_box(1));
        })
    );
}

pub fn multi_tree_count_bench(c: &mut Criterion) {
    let input = util::read_lines("inputs/day3_input.txt");
    let char_matrix = input_to_bitfields(&input);
    let slopes: Vec<(usize, usize)> = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    c.bench_function("Day 3 - Part 2",
     |b| b.iter(|| {
            multi_tree_count(black_box(&char_matrix), black_box(&slopes));
        })
    );
}

criterion_group!(benches, tree_count_bench, multi_tree_count_bench);
criterion_main!(benches);