use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_2020::day1::*;

pub fn complement_benchmark(c: &mut Criterion) {
    c.bench_function("Day 1 - Part 1",
     |b| b.iter(|| {
            let list = list_of_nums("inputs/day1_input.txt");
            complements(black_box(&list), black_box(2020));
        })
    );
}

pub fn three_complement_benchmark(c: &mut Criterion) {
    c.bench_function("Day 1 - Part 2",
     |b| b.iter(|| {
            let list = list_of_nums("inputs/day1_input.txt");
            three_complements(black_box(&list), black_box(2020));
        })
    );
}

criterion_group!(benches, complement_benchmark, three_complement_benchmark);
criterion_main!(benches);