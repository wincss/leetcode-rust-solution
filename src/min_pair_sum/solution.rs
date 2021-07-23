use crate::*;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut max = 0;
        for i in 0..(n / 2) {
            max = max.max(nums[i] + nums[n - 1 - i]);
        }
        max
    }
}
