//! # LeetCode Problem: 2940 - Find a Building Where Alice and Bob Can Meet
//!
//! Difficulty: Hard
//!
//! Link: https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n + q log q).
//! - Space Complexity: O(n + q).
//!
//! ## A bit of theory
//!
//! The problem is a variant of the "meeting point" problem, where we need
//! to find the leftmost building that can accommodate a meeting between
//! Alice and Bob.
//! The main approach is to use a priority queue (min-heap) to keep track of
//! the pending meeting requests and their required building heights.
//! We process the buildings from left to right, checking if the current
//! building can satisfy any of the pending requests.

pub struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Define a custom struct for pending meeting locations.
// This struct will hold the minimum required building height,
// the starting position, and the original query ID.
#[derive(PartialEq, Eq, Clone)]
struct MeetingRequest {
    min_building_height: i32,
    start_position: usize,
    original_query_id: usize,
}

// Implement comparison traits for priority queue ordering
impl Ord for MeetingRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use reverse ordering for min-heap behavior based on the required height
        other.min_building_height.cmp(&self.min_building_height)
    }
}

impl PartialOrd for MeetingRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let query_count = queries.len();
        let mut meeting_locations = vec![-1; query_count];
        let mut pending_requests = Vec::new();

        // Process each query and identify which need further processing
        for (query_id, query) in queries.iter().enumerate() {
            let (mut a, mut b) = (query[0] as usize, query[1] as usize);

            // Make sure person_a is at a position not exceeding person_b
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }

            // Handle easy cases: the same building or building B is already tall enough
            if a == b || heights[b] > heights[a] {
                meeting_locations[query_id] = b as i32;
            } else {
                // For other cases, we need a higher building than both current positions
                pending_requests.push(MeetingRequest {
                    min_building_height: heights[a] + 1, // Need a taller building
                    start_position: b,                   // Start searching from `b`
                    original_query_id: query_id,
                });
            }
        }

        // Sort pending requests by their starting position to process efficiently
        pending_requests.sort_by_key(|req| req.start_position);

        // Create a priority queue for active requests
        let mut active_requests = BinaryHeap::new();
        let mut request_index = 0;

        // Scan through buildings from left to right
        for (building_pos, &building_height) in heights.iter().enumerate() {
            // Add all requests that should start at or before this building
            while request_index < pending_requests.len()
                && pending_requests[request_index].start_position <= building_pos
            {
                active_requests.push(pending_requests[request_index].clone());
                request_index += 1;
            }

            // Check if this building satisfies any active requests
            while let Some(top_request) = active_requests.peek() {
                if top_request.min_building_height <= building_height {
                    // This building is tall enough for the meeting
                    let request = active_requests.pop().unwrap();
                    meeting_locations[request.original_query_id] = building_pos as i32;
                } else {
                    // Can't satisfy more requests with this building
                    break;
                }
            }
        }

        meeting_locations
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_leftmost_building_queries() {
        let test_cases = [
            (
                vec![6, 4, 8, 5, 2, 7],
                vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]],
                vec![2, 5, -1, 5, 2],
            ),
            (
                vec![5, 3, 8, 2, 6, 1, 4, 6],
                vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]],
                vec![7, 6, -1, 4, 6],
            ),
        ];
        for (idx, (heights, queries, expected)) in test_cases.iter().enumerate() {
            let result = Solution::leftmost_building_queries(heights.clone(), queries.clone());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with heights {heights:?}, queries {queries:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
