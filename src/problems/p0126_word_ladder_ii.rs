//! # LeetCode Problem: 126 - Word Ladder II
//!
//! Difficulty: Hard
//!
//! Link: https://leetcode.com/problems/word-ladder-ii/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n * m^2) - We iterate through the list containing n words,
//!   where m is the length of each word. For each word, we generate m patterns,
//!   and for each pattern, we check all n words to find neighbors.
//! - Space Complexity: O(n * m) - We use a map to store the patterns and
//!   their corresponding indices, which can grow linearly with the number of words.
pub struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        // Convert all words to byte arrays for faster processing
        let begin_bytes = begin_word.clone().into_bytes();
        let end_bytes = end_word.clone().into_bytes();
        let mut word_bytes: Vec<Vec<u8>> = word_list.into_iter().map(|w| w.into_bytes()).collect();

        // Check if the end word exists in the word list
        let end_idx = word_bytes.iter().position(|w| *w == end_bytes);
        if end_idx.is_none() {
            return vec![];
        }

        // Add begin word to the list if it's not already there
        let begin_idx = match word_bytes.iter().position(|w| *w == begin_bytes) {
            Some(idx) => idx,
            None => {
                word_bytes.push(begin_bytes.clone());
                word_bytes.len() - 1
            }
        };

        // Build word patterns map for faster neighbor finding
        let patterns = Self::build_patterns(&word_bytes);

        // Find the shortest paths using a BFS from beginning to end
        let paths = Self::find_paths(&patterns, &word_bytes, begin_idx, end_idx.unwrap());

        // Convert paths to strings with the original word order
        paths
            .into_iter()
            .map(|path| {
                path.into_iter()
                    .map(|idx| {
                        if idx == begin_idx {
                            begin_word.clone()
                        } else if idx == end_idx.unwrap() {
                            end_word.clone()
                        } else {
                            String::from_utf8(word_bytes[idx].clone()).unwrap()
                        }
                    })
                    .collect()
            })
            .collect()
    }

    // Creates a map of word patterns to word indices
    fn build_patterns(words: &[Vec<u8>]) -> HashMap<Vec<u8>, Vec<usize>> {
        let mut pattern_map: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();

        for (idx, word) in words.iter().enumerate() {
            for i in 0..word.len() {
                let mut pattern = word.clone();
                pattern[i] = 0; // Use 0 as a wildcard

                pattern_map.entry(pattern).or_default().push(idx);
            }
        }

        pattern_map
    }

    // Finds all shortest paths from begin_idx to end_idx
    fn find_paths(
        patterns: &HashMap<Vec<u8>, Vec<usize>>,
        words: &[Vec<u8>],
        begin_idx: usize,
        end_idx: usize,
    ) -> Vec<Vec<usize>> {
        let mut result = Vec::new();

        // Build an adjacency map for faster lookup
        let mut adj_map = vec![Vec::new(); words.len()];
        for i in 0..words.len() {
            let word = &words[i];
            for j in 0..word.len() {
                let mut pattern = word.clone();
                pattern[j] = 0; // Create a wildcard pattern

                if let Some(neighbors) = patterns.get(&pattern) {
                    for &neighbor in neighbors {
                        if neighbor != i {
                            // Don't add self-loops
                            adj_map[i].push(neighbor);
                        }
                    }
                }
            }
        }

        // Remove duplicates from adjacency lists
        for adj_list in &mut adj_map {
            adj_list.sort_unstable();
            adj_list.dedup();
        }

        // BFS to find distances from begin_idx to all reachable nodes
        let mut distances = vec![usize::MAX; words.len()];
        let mut queue = VecDeque::new();

        distances[begin_idx] = 0;
        queue.push_back(begin_idx);

        while let Some(node) = queue.pop_front() {
            if node == end_idx {
                break; // Found the shortest path to end
            }

            let dist = distances[node];
            for &neighbor in &adj_map[node] {
                if distances[neighbor] == usize::MAX {
                    distances[neighbor] = dist + 1;
                    queue.push_back(neighbor);
                }
            }
        }

        // If end_idx is not reachable
        if distances[end_idx] == usize::MAX {
            return result;
        }

        // DFS to find all shortest paths
        let mut path = Vec::new();
        Self::dfs(
            begin_idx,
            end_idx,
            &adj_map,
            &distances,
            &mut path,
            &mut result,
        );

        result
    }

    // DFS helper to build all shortest paths
    fn dfs(
        current: usize,
        target: usize,
        adj_map: &[Vec<usize>],
        distances: &[usize],
        path: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        // Add the current node to a path
        path.push(current);

        if current == target {
            // Found a path to target
            result.push(path.clone());
        } else {
            // Continue DFS with neighbors that are one step closer to target
            for &next in &adj_map[current] {
                if distances[next] == distances[current] + 1 {
                    Self::dfs(next, target, adj_map, distances, path, result);
                }
            }
        }

        // Backtrack
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_ladders() {
        let test_cases = [
            (
                "hit",
                "cog",
                vec!["hot", "dot", "dog", "lot", "log", "cog"],
                vec![
                    vec!["hit", "hot", "dot", "dog", "cog"],
                    vec!["hit", "hot", "lot", "log", "cog"],
                ],
            ),
            (
                "hit",
                "cog",
                vec!["hot", "dot", "dog", "lot", "log"],
                vec![],
            ),
        ];
        for (idx, (begin_word, end_word, word_list, expected)) in test_cases.iter().enumerate() {
            let word_list_str: Vec<String> = word_list.iter().map(|&s| s.to_string()).collect();
            let result =
                Solution::find_ladders(begin_word.to_string(), end_word.to_string(), word_list_str);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with begin_word {begin_word:?}, end_word {end_word:?}, word_list {word_list:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
