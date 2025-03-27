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

This repository is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Note that this license applies only to my solution code and not to the LeetCode problem statements themselves.

## Problems

🟢 1\.  [Two Sum](https://leetcode.com/problems/two-sum/) -> [Solution](src/problems/p0001_two_sum.rs)

🟢 645\. [Set Mismatch](https://leetcode.com/problems/set-mismatch/) -> [Solution](src/problems/p0645_set_mismatch.rs)

🟢
1137\. [N-th Tribonacci Number](https://leetcode.com/problems/n-th-tribonacci-number/) -> [Solution](src/problems/p1137_nth_tribonacci_number.rs)

🟢
1784\. [Check if Binary String Has at Most One Segment of Ones](https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/) -> [Solution](src/problems/p1784_check_if_binary_string_has_at_most_one_segment_of_ones.rs)

🟢
1876\. [Substrings of Size Three with Distinct Characters](https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/) -> [Solution](src/problems/p1876_substrings_of_size_three_with_distinct_characters.rs)

🟢
1974\. [Minimum Time to Type Word Using Special Typewriter](https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/) -> [Solution](src/problems/p1974_minimimum_time_to_type_word_using_special_typewriter.rs)

🟢
2099\. [Find Subsequence of Length K With the Largest Sum](https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/) -> [Solution](src/problems/p2099_find_subsequence_of_length_k_with_the_largest_sum.rs)

🟢
2399\. [Check Distances Between Same Letters](https://leetcode.com/problems/check-distances-between-same-letters/) -> [Solution](src/problems/p2399_check_distances_between_same_letters.rs)

🟢 3174\. [Clear digits](https://leetcode.com/problems/clear-digits/) -> [Solution](src/problems/p3174_clear_digits.rs)

## How to add a new problem

1. Create a new file in `src/` directory with the name of the problem in snake_case with format
   `p<XXXX>_problem_title.rs`.
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
mod p < XXXX>_problem_title;
```

where `<XXXX>` is the number of the problem on LeetCode and `problem_title` is the name of the problem in snake_case.

4. Add the problem to the list in `README.md` with the following format:

```markdown
N\. [Problem Title](https://leetcode.com/problems/problem-slug/) -> [Solution](src/problems/pXXXX_problem_title.rs)
```

where N is the number of the problem on LeetCode.

Backslash is used to escape the dot in the list item to prevent it from being interpreted as a numbered list,
as markdown will automatically renumber the list items.

Emojis are used to indicate the difficulty level of the problem:

- 🟢 Easy
- 🟠 Medium
- 🔴 Hard

5. Run the tests with `cargo test` to make sure everything is working.
6. Commit your changes.
