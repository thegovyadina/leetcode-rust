# LeetCode solutions in Rust

[![Rust checks and tests](https://github.com/thegovyadina/leetcode-rust/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/thegovyadina/leetcode-rust/actions/workflows/rust.yml)
[![Commit Activity](https://img.shields.io/github/commit-activity/m/thegovyadina/leetcode-rust)](https://github.com/thegovyadina/leetcode-rust/commits/main)


<!-- LEETCODE-BADGES:START -->
[![Easy](https://img.shields.io/badge/Easy-20-brightgreen)](#problems) [![Medium](https://img.shields.io/badge/Medium-21-orange)](#problems) [![Hard](https://img.shields.io/badge/Hard-6-red)](#problems) [![Total](https://img.shields.io/badge/Total-47-blue)](#problems)
<!-- LEETCODE-BADGES:END -->

## Disclaimer

This repository contains my personal solutions to LeetCode problems, created for educational purposes
and personal reference only. All problem descriptions, names, and related content
are the property of LeetCode (https://leetcode.com/).

- The solutions provided here are meant for learning and study purposes only
- Problem statements are intentionally omitted or minimized to avoid copyright issues
- Links to original problems are provided for proper reference
- This is not an official LeetCode repository and is not affiliated with LeetCode

If you're a LeetCode representative and have concerns about this repository,
please contact me and I'll address them promptly.

## License

This repository is licensed under the MIT License—see the [LICENSE](LICENSE) file for details.

Note that this license applies only to my solution code and not to the LeetCode problem statements themselves.

## Problems

| Problem                                                                                                                                                            |                                        Solution                                         | Approach                           |
|:-------------------------------------------------------------------------------------------------------------------------------------------------------------------|:---------------------------------------------------------------------------------------:|:-----------------------------------|
| 🟢 1. [Two Sum](https://leetcode.com/problems/two-sum/)                                                                                                            |                           [🦀](src/problems/p0001_two_sum.rs)                           | Hash Map                           |
| 🟠 3. [Longest Substring Without Repeating Characters](https://leetcode.com/problems/longest-substring-without-repeating-characters/)                              |       [🦀](src/problems/p0003_longest_substring_without_repeating_characters.rs)        | HashMap, Two Pointers              |
| 🟢 13. [Roman to Integer](https://leetcode.com/problems/roman-to-integer/)                                                                                         |                      [🦀](src/problems/p0013_roman_to_integer.rs)                       |                                    |
| 🟢 20. [Valid Parentheses](https://leetcode.com/problems/valid-parentheses/)                                                                                       |                      [🦀](src/problems/p0020_valid_parentheses.rs)                      | Stack                              |
| 🟢 21. [Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)                                                                             |                   [🦀](src/problems/p0021_merge_two_sorted_lists.rs)                    |                                    |
| 🟠 33. [Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)                                                             |               [🦀](src/problems/p0033_search_in_rotated_sorted_array.rs)                | Binary search                      |
| 🟠 39. [Combination Sum](https://leetcode.com/problems/combination-sum/)                                                                                           |                       [🦀](src/problems/p0039_combination_sum.rs)                       | Backtracking                       |
| 🟠 96. [Unique Binary Search Trees](https://leetcode.com/problems/unique-binary-search-trees/)                                                                     |                 [🦀](src/problems/p0096_unique_binary_search_trees.rs)                  | Dynamic Programming                |
| 🔴 126. [Word Ladder II](https://leetcode.com/problems/word-ladder-ii/)                                                                                            |                       [🦀](src/problems/p0126_word_ladder_ii.rs)                        | Breadth-First Search               |
| 🟠 215. [Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)                                                          |               [🦀](src/problems/p0215_kth_largest_element_in_an_array.rs)               |                                    |
| 🟠 227. [Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)                                                                                  |                     [🦀](src/problems/p0227_basic_calculator_2.rs)                      | Stack                              |
| 🟢 263. [Ugly Number](https://leetcode.com/problems/ugly-number/)                                                                                                  |                         [🦀](src/problems/p0263_ugly_number.rs)                         |                                    |
| 🟢 504. [Base 7](https://leetcode.com/problems/base-7/)                                                                                                            |                           [🦀](src/problems/p0504_base_7.rs)                            |                                    |
| 🔴 552. [Student Attendance Record II](https://leetcode.com/problems/student-attendance-record-ii/)                                                                |                [🦀](src/problems/p0552_student_attendance_record_ii.rs)                 | Dynamic Programming                |
| 🟢 599. [Minimum Index Sum of Two Lists](https://leetcode.com/problems/minimum-index-sum-of-two-lists/)                                                            |               [🦀](src/problems/p0599_minimum_index_sum_of_two_lists.rs)                | HashMap                            |
| 🟢 645. [Set Mismatch](https://leetcode.com/problems/set-mismatch/)                                                                                                |                        [🦀](src/problems/p0645_set_mismatch.rs)                         |                                    |
| 🟠 707. [Design Linked List](https://leetcode.com/problems/design-linked-list/)                                                                                    |                     [🦀](src/problems/p0707_design_linked_list.rs)                      |                                    |
| 🟠 739. [Daily Temperatures](https://leetcode.com/problems/daily-temperatures/)                                                                                    |                     [🦀](src/problems/p0739_daily_temperatures.rs)                      | Monotonic stack                    |
| 🟠 797. [All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)                                                          |               [🦀](src/problems/p0797_all_paths_from_source_to_target.rs)               | Depth-First Search                 |
| 🟢 905. [Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity/)                                                                                |                    [🦀](src/problems/p0905_sort_array_by_parity.rs)                     |                                    |
| 🟠 954. [Array of Doubled Pairs](https://leetcode.com/problems/array-of-doubled-pairs/)                                                                            |                   [🦀](src/problems/p0954_array_of_doubled_pairs.rs)                    |                                    |
| 🟠 957. [Prison Cells After N Days](https://leetcode.com/problems/prison-cells-after-n-days/)                                                                      |                  [🦀](src/problems/p0957_prison_cells_after_n_days.rs)                  |                                    |
| 🟢 1137. [N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/)                                                                           |                    [🦀](src/problems/p1137_nth_tribonacci_number.rs)                    |                                    |
| 🟢 1431. [Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)                                       |          [🦀](src/problems/p1431_kids_with_the_greatest_number_of_candies.rs)           |                                    |
| 🟢 1464. [Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/)                                 |         [🦀](src/problems/p1464_maximum_product_of_two_elements_in_an_array.rs)         |                                    |
| 🟠 1599. [Maximum Profit of Operating a Centennial Wheel](https://leetcode.com/problems/maximum-profit-of-operating-a-centennial-wheel/)                           |       [🦀](src/problems/p1599_maximum_profit_of_operating_a_centennial_wheel.rs)        |                                    |
| 🟢 1784. [Check if Binary String Has at Most One Segment of Ones](https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/)           |   [🦀](src/problems/p1784_check_if_binary_string_has_at_most_one_segment_of_ones.rs)    |                                    |
| 🟠 1814. [Count Nice Pairs in an Array](https://leetcode.com/problems/count-nice-pairs-in-an-array/)                                                               |                [🦀](src/problems/p1814_count_nice_pairs_in_an_array.rs)                 | HashMap                            |
| 🔴 1866. [Number of Ways to Rearrange Sticks With K Sticks Visible](https://leetcode.com/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/)       |  [🦀](src/problems/p1866_number_of_ways_to_rearrange_sticks_with_k_sticks_visible.rs)   | Dynamic Programming                |
| 🟢 1876. [Substrings of Size Three with Distinct Characters](https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/)                     |      [🦀](src/problems/p1876_substrings_of_size_three_with_distinct_characters.rs)      |                                    |
| 🟢 1974. [Minimum Time to Type Word Using Special Typewriter](https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/)                   |    [🦀](src/problems/p1974_minimimum_time_to_type_word_using_special_typewriter.rs)     |                                    |
| 🟢 2099. [Find Subsequence of Length K With the Largest Sum](https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/)                     |      [🦀](src/problems/p2099_find_subsequence_of_length_k_with_the_largest_sum.rs)      |                                    |
| 🟠 2201. [Count Artifacts That Can Be Extracted](https://leetcode.com/problems/count-artifacts-that-can-be-extracted/)                                             |            [🦀](src/problems/p2201_count_artifacts_that_can_be_extracted.rs)            |                                    |
| 🟠 2270. [Number of Ways to Split Array](https://leetcode.com/problems/number-of-ways-to-split-array/)                                                             |                [🦀](src/problems/p2270_number_of_ways_to_split_array.rs)                |                                    |
| 🔴 2338. [Count the Number of Ideal Arrays](https://leetcode.com/problems/count-the-number-of-ideal-arrays/)                                                       |              [🦀](src/problems/p2338_count_the_number_of_ideal_arrays.rs)               | Dynamic Programming, Combinatorics |
| 🟢 2399. [Check Distances Between Same Letters](https://leetcode.com/problems/check-distances-between-same-letters/)                                               |            [🦀](src/problems/p2399_check_distances_between_same_letters.rs)             |                                    |
| 🟠 2442. [Count Number of Distinct Integers After Reverse Operations](https://leetcode.com/problems/count-number-of-distinct-integers-after-reverse-operations/)   | [🦀](src/problems/p2442_count_number_of_distinct_integers_after_reverse_operations.rs)  | Hash Set                           |
| 🟠 2471. [Minimum Number of Operations to Sort a Binary Tree by Level](https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/) | [🦀](src/problems/p2471_minimum_number_of_operations_to_sort_a_binary_tree_by_level.rs) |                                    |
| 🟠 2857. [Count Pairs of Points With Distance k](https://leetcode.com/problems/count-pairs-of-points-with-distance-k/)                                             |            [🦀](src/problems/p2471_count_pairs_of_points_with_distance_k.rs)            | Hash Map                           |
| 🔴 2902. [Count of Sub-Multisets With Bounded Sum](https://leetcode.com/problems/count-of-sub-multisets-with-bounded-sum/)                                         |           [🦀](src/problems/p2902_count_of_sub_multisets_with_bounded_sum.rs)           | Dynamic Programming                |
| 🟠 2933. [High-Access Employees](https://leetcode.com/problems/high-access-employees/)                                                                             |                    [🦀](src/problems/p2933_high_access_employees.rs)                    | Hash Map                           |
| 🔴 2940. [Find Building Where Alice and Bob Can Meet](https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/)                                   |         [🦀](src/problems/p2940_find_building_where_alice_and_bob_can_meet.rs)          | Priority Queue                     |
| 🟠 3021. [Alice and Bob Playing Flower Game](https://leetcode.com/problems/alice-and-bob-playing-flower-game/)                                                     |              [🦀](src/problems/p3021_alice_and_bob_playing_flower_game.rs)              |                                    |
| 🟢 3174. [Clear digits](https://leetcode.com/problems/clear-digits/)                                                                                               |                        [🦀](src/problems/p3174_clear_digits.rs)                         | Stack                              |
| 🟠 3208. [Alternating Groups II](https://leetcode.com/problems/alternating-groups-ii/)                                                                             |                    [🦀](src/problems/p3208_alternating_groups_ii.rs)                    | Sliding Window                     |
| 🟠 3212. [Count Submatrices With Equal Frequency of X and Y](https://leetcode.com/problems/count-submatrices-with-equal-frequency-of-x-and-y/)                     |      [🦀](src/problems/p3212_count_submatrices_with_equal_frequency_of_x_and_y.rs)      | Dynamic Programming                |
| 🟢 3270. [Find the Key of the Numbers](https://leetcode.com/problems/find-the-key-of-the-numbers/)                                                                 |                 [🦀](src/problems/p3270_find_the_key_of_the_numbers.rs)                 |                                    |
| 🟢 3289. [The Two Sneaky Numbers of Digitville](https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/)                                               |            [🦀](src/problems/p3289_the_two_sneaky_numbers_of_digitville.rs)             |                                    |

## Development Setup

After cloning the repository, run this command to set up Git hooks:

```bash
git config core.hooksPath .githooks
```

## How to add a new problem

1. Create a new file in `src/problems/` directory with the name of the problem in snake_case with format
   `pNNNN_problem_title.rs`, where `NNNN` is the number of the problem on LeetCode in 4-digit format with leading zeros
   and `problem_title` is the name of the problem in snake_case.

2. Use the following template for the file:

```rust
//! # LeetCode Problem: [Problem Number] - [Problem Title]
//!
//! Difficulty: [Easy/Medium/Hard]
//!
//! Link: https://leetcode.com/problems/[problem-slug]/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - Describe the time complexity of your solution.
//! - Space Complexity: O(n) - Describe the space complexity of your solution.

pub struct Solution;

// Copy the problem title from LeetCode
impl Solution {
    pub fn problem_title() {
        // Your solution here
    }
}

// Tests are written in the same file and are mandatory
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_problem_title() {
        // Your tests here
    }
}
```

Complexity analysis is optional but recommended. If you're not sure about the complexity of your solution,
you can leave it out.

3. Update the `lib.rs` file to include the new problem file. Add the following line at the end of the file:

```rust
pub mod pNNNN_problem_title;
```

4. Add the problem to the [Problems](#problems) table in `README.md`
5. Run the tests with `cargo test` to make sure everything is working.
6. Commit your changes.

## Benchmarking

For benchmarking, the `criterion` crate is used.

To add a new benchmark, create a new file in the [/benches/](/benches/) directory and add a new `[[branch]]` section
in [Cargo.toml](Cargo.toml).

To execute a benchmark, run one of the following commands:

```bash
# Run all benchmarks
cargo bench

# Run all benchmarks defined in a specific file
cargo bench --bench p1814
```

Also, you can run them more precisely using special syntax. E.g.:

```
# Run only the benchmark for the specific number 1234
cargo bench --bench p1814 "loop_based_1234"

# Run all loop-based benchmarks
cargo bench --bench p1814 "loop_based_.*"
```

See the [Criterion documentation](https://bheisler.github.io/criterion.rs/book/user_guide/known_limitations.html)
for more information or run `cargo help bench`.
