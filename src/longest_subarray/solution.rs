use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut right = 0;
        let mut result = 0;
        let mut max = VecDeque::new();
        let mut min = VecDeque::new();
        for left in 0..n {
            while let Some(&v) = max.front() {
                if v < left {
                    max.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&v) = min.front() {
                if v < left {
                    min.pop_front();
                } else {
                    break;
                }
            }
            while right < n {
                while let Some(&v) = max.back() {
                    if nums[v] < nums[right] {
                        max.pop_back();
                    } else {
                        break;
                    }
                }
                max.push_back(right);
                while let Some(&v) = min.back() {
                    if nums[v] > nums[right] {
                        min.pop_back();
                    } else {
                        break;
                    }
                }
                min.push_back(right);
                right += 1;
                let min_idx = min.front().map_or(0, |&v| v);
                let max_idx = max.front().map_or(0, |&v| v);
                if nums[max_idx] - nums[min_idx] > limit {
                    break;
                }
                result = result.max(right - left);
            }
        }
        result as i32
    }
}
