# LeetCode solutions in Rust

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

| Difficulty | Problem<br/>Number | Problem Title                                                                                                                                   | Solution                                                                                                                                        |
|:----------:|:-------------------|:------------------------------------------------------------------------------------------------------------------------------------------------|:------------------------------------------------------------------------------------------------------------------------------------------------|
|     🟢     | 1                  | [Two Sum](https://leetcode.com/problems/two-sum/)                                                                                               | [p0001_two_sum.rs](src/problems/p0001_two_sum.rs)                                                                                               |
|     🟠     | 227                | [Basic Calculator II](https://leetcode.com/problems/basic-calculator-ii/)                                                                       | [p0227_basic_calculator_2.rs](src/problems/p0227_basic_calculator_2.rs)                                                                         |
|     🟢     | 645                | [Set Mismatch](https://leetcode.com/problems/set-mismatch/)                                                                                     | [p0645_set_mismatch.rs](src/problems/p0645_set_mismatch.rs)                                                                                     |
|     🟠     | 797                | [All Paths From Source to Target](https://leetcode.com/problems/all-paths-from-source-to-target/)                                               | [p0797_all_paths_from_source_to_target.rs](src/problems/p0797_all_paths_from_source_to_target.rs)                                               |
|     🟢     | 905                | [Sort Array By Parity](https://leetcode.com/problems/sort-array-by-parity/)                                                                     | [p0905_sort_array_by_parity.rs](src/problems/p0905_sort_array_by_parity.rs)                                                                     |
|     🟢     | 1137               | [N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/)                                                                 | [p1137_nth_tribonacci_number.rs](src/problems/p1137_nth_tribonacci_number.rs)                                                                   |
|     🟢     | 1431               | [Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)                             | [p1431_kids_with_the_greatest_number_of_candies.rs](src/problems/p1431_kids_with_the_greatest_number_of_candies.rs)                             |
|     🟢     | 1464               | [Maximum Product of Two Elements in an Array](https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/)                       | [p1464_maximum_product_of_two_elements_in_an_array.rs](src/problems/p1464_maximum_product_of_two_elements_in_an_array.rs)                       |
|     🟢     | 1784               | [Check if Binary String Has at Most One Segment of Ones](https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/) | [p1784_check_if_binary_string_has_at_most_one_segment_of_ones.rs](src/problems/p1784_check_if_binary_string_has_at_most_one_segment_of_ones.rs) |
|     🟢     | 1876               | [Substrings of Size Three with Distinct Characters](https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/)           | [p1876_substrings_of_size_three_with_distinct_characters.rs](src/problems/p1876_substrings_of_size_three_with_distinct_characters.rs)           |
|     🟢     | 1974               | [Minimum Time to Type Word Using Special Typewriter](https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/)         | [p1974_minimimum_time_to_type_word_using_special_typewriter.rs](src/problems/p1974_minimimum_time_to_type_word_using_special_typewriter.rs)     |
|     🟢     | 2099               | [Find Subsequence of Length K With the Largest Sum](https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/)           | [p2099_find_subsequence_of_length_k_with_the_largest_sum.rs](src/problems/p2099_find_subsequence_of_length_k_with_the_largest_sum.rs)           |
|     🔴     | 2338               | [Count the Number of Ideal Arrays](https://leetcode.com/problems/count-the-number-of-ideal-arrays/)                                             | [p2338_count_the_number_of_ideal_arrays.rs](src/problems/p2338_count_the_number_of_ideal_arrays.rs)                                             |
|     🟢     | 2399               | [Check Distances Between Same Letters](https://leetcode.com/problems/check-distances-between-same-letters/)                                     | [p2399_check_distances_between_same_letters.rs](src/problems/p2399_check_distances_between_same_letters.rs)                                     |
|     🟢     | 3174               | [Clear digits](https://leetcode.com/problems/clear-digits/)                                                                                     | [p3174_clear_digits.rs](src/problems/p3174_clear_digits.rs)                                                                                     |
|     🟢     | 3270               | [Find the Key of the Numbers](https://leetcode.com/problems/find-the-key-of-the-numbers/)                                                       | [p3270_find_the_key_of_the_numbers.rs](src/problems/p3270_find_the_key_of_the_numbers.rs)                                                       |

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
