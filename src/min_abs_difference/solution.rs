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

        let mut leftsum = vec![0; 1 << left];
        for i in 1..(1 << left) {
            let key = i & (i - 1);
            leftsum[i] = leftsum[key] + nums[bitmap[&(i - key)]];
        }
        leftsum.sort();
        let num_leftsum = leftsum.len();

        let mut rightsum = vec![0; 1 << right];
        let mut mindiff = std::i32::MAX;
        for i in 0..(1 << right) {
            let v = if i > 0 {
                let key = i & (i - 1);
                rightsum[key] + nums[left + bitmap[&(i - key)]]
            } else {
                0
            };
            rightsum[i] = v;

            let pos = leftsum.binary_search(&(goal - v)).unwrap_or_else(|x| x);
            if pos < num_leftsum {
                mindiff = std::cmp::min(mindiff, (goal - v - leftsum[pos]).abs());
            }
            if pos > 0 {
                mindiff = std::cmp::min(mindiff, (goal - v - leftsum[pos - 1]).abs());
            }
        }
        // println!("{:?} {:?}", leftsum, rightsum);
        mindiff
    }
}
