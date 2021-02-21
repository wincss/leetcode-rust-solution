use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut result = 0;
        let mut max = VecDeque::new();
        let mut min = VecDeque::new();
        for right in 0..n {
            while !max.is_empty() && nums[*max.back().unwrap()] < nums[right] {
                max.pop_back();
            }
            max.push_back(right);
            while !min.is_empty() && nums[*min.back().unwrap()] > nums[right] {
                min.pop_back();
            }
            min.push_back(right);
            loop {
                let min_idx = min.front().map_or(0, |&v| v);
                let max_idx = max.front().map_or(0, |&v| v);
                if nums[max_idx] - nums[min_idx] <= limit {
                    break;
                }
                left += 1;
                while !max.is_empty() && *max.front().unwrap() < left {
                    max.pop_front();
                }
                while !min.is_empty() && *min.front().unwrap() < left {
                    min.pop_front();
                }
            }
            result = result.max(right - left + 1);
        }
        result as i32
    }
}
