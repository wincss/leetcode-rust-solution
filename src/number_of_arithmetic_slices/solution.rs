use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn number_of_arithmetic_slices_446(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }
        let mut dp = vec![HashMap::new(); n];
        let mut result = 0;
        for i in 0..n {
            for j in 0..i {
                if let Some(d) = nums[i].checked_sub(nums[j]) {
                    let count = *dp[j].entry(d).or_insert(0);
                    result += count;
                    *dp[i].entry(d).or_insert(0) += count + 1;
                }
            }
        }
        result
    }
    pub fn number_of_arithmetic_slices_413(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        let mut result = 0;
        let mut diff = None;
        let mut length = 1;
        for i in 1..n {
            if diff.is_none() {
                diff = Some(nums[i] - nums[i - 1]);
                length = 2;
            } else if nums[i] - nums[i - 1] == diff.unwrap() {
                length += 1;
            } else {
                result += (length - 1) * (length - 2) / 2;
                diff = Some(nums[i] - nums[i - 1]);
                length = 2;
            }
        }
        result += (length - 1) * (length - 2) / 2;
        result
    }
}
