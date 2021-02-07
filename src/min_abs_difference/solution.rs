use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        if n == 1 {
            return (goal - nums[0]).abs();
        }
        let mut nums = nums;
        nums.sort();
        let left = n / 2;
        let right = n - left;
        let mut leftsum = HashSet::new();
        for i in 0..(1 << left) {
            let mut s = 0;
            for j in 0..left {
                if i & (1 << j) > 0 {
                    s += nums[j];
                }
            }
            leftsum.insert(s);
        }
        let mut leftsum: Vec<i32> = leftsum.into_iter().collect();
        leftsum.sort();
        let lsn = leftsum.len();
        let mut mindiff = std::i32::MAX;
        for i in 0..(1 << right) {
            let mut s = 0;
            for j in 0..right {
                if i & (1 << j) > 0 {
                    s += nums[left + j];
                }
            }
            let mut pos = leftsum.binary_search(&(goal - s)).unwrap_or_else(|x| x);
            let pos2 = if pos == lsn {
                pos = lsn - 1;
                lsn - 1
            } else if pos > 0 {
                pos - 1
            } else {
                0
            };
            mindiff = std::cmp::min(mindiff, (goal - s - leftsum[pos]).abs());
            mindiff = std::cmp::min(mindiff, (goal - s - leftsum[pos2]).abs());
        }
        mindiff
    }
}
