// benches/p1814
use criterion::{criterion_group, criterion_main, Criterion};
use leetcode_rust::problems::p1814_count_nice_pairs_in_an_array::Solution;
use std::hint::black_box;

fn bench_count_nice_pairs(c: &mut Criterion) {
    let nums = vec![42, 11, 1, 97];

    c.bench_function("count_nice_pairs", |b| {
        b.iter(|| Solution::count_nice_pairs(black_box(nums.clone())))
    });
}

fn bench_rev_implementations(c: &mut Criterion) {
    let mut group = c.benchmark_group("rev_implementations");

    // Implementation 1: Loop-based
    fn rev_loop(mut num: i32) -> i32 {
        let mut result = 0;
        while num > 0 {
            result = result * 10 + num % 10;
            num /= 10;
        }
        result
    }

    // Implementation 2: String-based
    fn rev_string(num: i32) -> i32 {
        num.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    let test_numbers = [42, 1234, 9876543, 1_000_000_000];

    for &num in &test_numbers {
        group.bench_function(format!("loop_based_{num}"), |b| {
            b.iter(|| rev_loop(black_box(num)))
        });

        group.bench_function(format!("string_based_{num}"), |b| {
            b.iter(|| rev_string(black_box(num)))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_count_nice_pairs, bench_rev_implementations);
criterion_main!(benches);
