use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        if n == 1 {
            return (goal - nums[0]).abs();
        }
        let mut nums = nums;
        nums.sort();
        let mut bitmap = HashMap::new();
        for i in 0_usize..32 {
            bitmap.insert(1 << i, i);
        }
        let left = n / 2;
        let right = n - left;

        let mut left_sum = vec![0; 1 << left];
        for i in 1..(1 << left) {
            let key = i & (i - 1);
            left_sum[i] = left_sum[key] + nums[bitmap[&(i - key)]];
        }
        left_sum.sort();
        let mut right_sum = vec![0; 1 << right];
        for i in 1..(1 << right) {
            let key = i & (i - 1);
            right_sum[i] = right_sum[key] + nums[left + bitmap[&(i - key)]];
        }
        right_sum.sort();

        let mut left_idx = 0;
        let mut right_idx = right_sum.len() - 1;
        let mut min_diff = std::i32::MAX;
        while left_idx < left_sum.len() {
            let total = left_sum[left_idx] + right_sum[right_idx];
            min_diff = std::cmp::min(min_diff, (total - goal).abs());

            if total > goal {
                if right_idx == 0 {
                    break;
                }
                right_idx -= 1;
            } else {
                left_idx += 1;
            }
        }
        min_diff
    }
}
