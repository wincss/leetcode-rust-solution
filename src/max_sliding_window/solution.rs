use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut w = VecDeque::new();
        let mut result = Vec::new();
        for i in 0..n {
            while let Some(&idx) = w.front() {
                if idx + k <= i {
                    w.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&idx) = w.back() {
                if nums[i] > nums[idx] {
                    w.pop_back();
                } else {
                    break;
                }
            }
            w.push_back(i);
            if i >= k - 1 {
                result.push(nums[*w.front().unwrap()]);
            }
        }
        result
    }
}
