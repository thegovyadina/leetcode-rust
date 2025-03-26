# LeetCode solutions in Rust

## Problems

1\. [Two Sum](https://leetcode.com/problems/two-sum/)

645\. [Set Mismatch](https://leetcode.com/problems/set-mismatch/)

## How to add a new problem

1. Create a new file in `src/` directory with the name of the problem in snake_case with format
   `p<XXXX>_problem_title.rs`.
2. Use the following template for the file:

```rust
//! # LeetCode Problem: [Problem Number] - [Problem Title]
//!
//! [Problem Description Placeholder]
//!
//! Difficulty: [Easy/Medium/Hard]
//!
//! Link: https://leetcode.com/problems/[problem-slug]/

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

3. Update the `lib.rs` file to include the new problem file. Add the following line at the end of the file:

```rust
mod p < XXXX>_problem_title;
```

where `<XXXX>` is the number of the problem on LeetCode and `problem_title` is the name of the problem in snake_case.

4. Add the problem to the list in `README.md` with the following format:

```markdown
N\. [Problem Title](https://leetcode.com/problems/problem-slug/)
```

where N is the number of the problem on LeetCode.

Backslash is used to escape the dot in the list item to prevent it from being interpreted as a numbered list,
as markdown will automatically renumber the list items.

5. Run the tests with `cargo test` to make sure everything is working.
6. Commit your changes.
