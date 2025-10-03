// benches/p1910
use criterion::{criterion_group, criterion_main, Criterion};
use leetcode_rust::problems::p1910_remove_all_occurrences_of_a_substring::Solution;
use std::hint::black_box;

fn bench_remove_occurrences(c: &mut Criterion) {
    let s = "daabcbaabcbcabchwswhabcafdwecabcabcabcwddnewwndewndjwcabcabcabcwddnewwndewndcabcabcabcwddnewwndewndjwenjdewjwenjdewenjdewnhdwehqhufebrhfberkwbfhkewnjkewnhjkfqfkwrjijif3fednavjkbhewkqfndwjebnfhkqwbfhkebwhqfbejqewbfhcdjhgfyqwkncjerlqnwqfukerbcabcabcabcwddnewwndewndjwenjdewygfhwuejdwqebferwhfqhyieruwhfnewjldnukbgrfekndjkwqengktrnfwkcabcabcabcwcabcabcabcwddnewwndewndjwenjdewddnewwndewndjwenjdewejdjlw;fkr9pewpquaavhbndlkmcdkj.dsfsbcnjwenvkhceasjncjkewrnvkhjecabcabcabcwddnewwcabcabcabcwddnewwncabcabcabcwddnewwndewndjwcabcabcabcwddnewwndewndjwenjdewenjdewdewndjwenjdewndewndjwenjdewrbvhkerbvhkbenrjkvneraw.vnerjkbvjkervnjerdbacabcabcfewabcabcabcabcacabfercabcabcabcabcbacaafweewcbabcabcabacbcabcabcabcababcbacbabcabcbcacabcbabcabcabcbabcaba".to_string();
    let part = "cabcabcabcwddnewwndewndjwenjdew".to_string();

    c.bench_function("remove_occurrences", |b| {
        b.iter(|| Solution::remove_occurrences(black_box(s.clone()), black_box(part.clone())))
    });

    c.bench_function("remove_occurrences_stack", |b| {
        b.iter(|| Solution::remove_occurrences_stack(black_box(s.clone()), black_box(part.clone())))
    });
}

criterion_group!(benches, bench_remove_occurrences);
criterion_main!(benches);
