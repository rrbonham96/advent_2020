use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_2020::day2::*;
use advent_2020::util;

pub fn password_benchmark(c: &mut Criterion) {
    let input = util::load_file("inputs/day2_input.txt");
    let lines = input.split('\n').map(|s| s.to_string()).collect();
    c.bench_function("Day 2 - Part 1",
     |b| b.iter(|| {
            valid_passwords(black_box(&lines));
        })
    );
}

pub fn actual_password_benchmark(c: &mut Criterion) {
    let input = util::load_file("inputs/day2_input.txt");
    let lines = input.split('\n').map(|s| s.to_string()).collect();
    c.bench_function("Day 2 - Part 2",
     |b| b.iter(|| {
            actually_valid_passwords(black_box(&lines));
        })
    );
}

criterion_group!(benches, password_benchmark, actual_password_benchmark);
criterion_main!(benches);