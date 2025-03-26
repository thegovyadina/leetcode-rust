LeetCode solutions in Rust

## Problems

- [Two Sum](src/problems/p0001_two_sum.rs)

## How to add a new problem

1. Create a new file in `src/` directory with the name of the problem in snake_case with format
   `p<XXXX>_problem_title.rs`.
2. Use the following template for the file:

```rust
pub struct Solution;

// Copy the problem title from LeetCode
impl Solution {
    pub fn problem_title() {
        // Your solution here
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_problem_title() {
        // Your tests here
    }
}
```

3. Add the problem to the list in `README.md` with the following format:

```markdown
- [Problem Title](src/p<XXXX>_problem_title.rs)
```

4. Run the tests with `cargo test` to make sure everything is working.
5. Commit your changes.
