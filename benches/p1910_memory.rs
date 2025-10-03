// benches/p1910_memory.rs
use criterion::{criterion_group, criterion_main, Criterion};
use leetcode_rust::problems::p1910_remove_all_occurrences_of_a_substring::Solution;
use std::hint::black_box;

fn bench_memory_usage(c: &mut Criterion) {
    // Get implementation choice from environment (1=standard, 2=stack)
    let impl_choice = std::env::var("IMPL").unwrap_or_else(|_| "1".to_string());

    // Larger input for more noticeable memory patterns
    let base_s = "daabcbaabcbcabchwswhabcafdwecabcabcabcwddnewwndewndjwcabcabcabcwddnewwndewndcabcabcabcwddnewwndewndjwenjdewjwenjdewenjdewnhdwehqhufebrhfberkwbfhkewnjkewnhjkfqfkwrjijif3fednavjkbhewkqfndwjebnfhkqwbfhkebwhqfbejqewbfhcdjhgfyqwkncjerlqnwqfukerbcabcabcabcwddnewwndewndjwenjdewygfhwuejdwqebferwhfqhyieruwhfnewjldnukbgrfekndjkwqengktrnfwkcabcabcabcwcabcabcabcwddnewwndewndjwenjdewddnewwndewndjwenjdewejdjlw;fkr9pewpquaavhbndlkmcdkj.dsfsbcnjwenvkhceasjncjkewrnvkhjecabcabcabcwddnewwcabcabcabcwddnewwncabcabcabcwddnewwndewndjwcabcabcabcwddnewwndewndjwenjdewenjdewdewndjwenjdewndewndjwenjdewrbvhkerbvhkbenrjkvneraw.vnerjkbvjkervnjerdbacabcabcfewabcabcabcabcacabfercabcabcabcabcbacaafweewcbabcabcabacbcabcabcabcababcbacbabcabcbcacabcbabcabcabcbabcaba";
    let part = "cabcabcabcwddnewwndewndjwenjdew";

    // Repeat string to make memory usage more pronounced
    let repeat = 1000;
    let s = base_s.repeat(repeat);
    let part = part.to_string();

    println!("Testing with string length: {}", s.len());

    if impl_choice == "1" {
        println!("Testing standard implementation");
        c.bench_function("remove_occurrences_memory", |b| {
            b.iter(|| Solution::remove_occurrences(black_box(s.clone()), black_box(part.clone())))
        });
    } else {
        println!("Testing stack implementation");
        c.bench_function("remove_occurrences_stack_memory", |b| {
            b.iter(|| {
                Solution::remove_occurrences_stack(black_box(s.clone()), black_box(part.clone()))
            })
        });
    }
}

criterion_group!(benches, bench_memory_usage);
criterion_main!(benches);
