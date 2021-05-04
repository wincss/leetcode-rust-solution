use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut v = HashMap::new();
        for i in nums {
            *v.entry(i).or_insert(0) += 1;
        }
        let mut nums: Vec<i32> = v.keys().map(|&v| v).collect();
        nums.sort();
        let mut last_keep = 0;
        let mut last_skip = 0;
        for i in (0..nums.len()).rev() {
            if i < nums.len() - 1 && nums[i] == nums[i + 1] - 1 {
                let tmp = nums[i] * v[&nums[i]] + last_skip;
                last_skip = last_skip.max(last_keep);
                last_keep = tmp;
            } else {
                last_skip = last_skip.max(last_keep);
                last_keep = nums[i] * v[&nums[i]] + last_skip;
            }
        }
        last_skip.max(last_keep)
    }
}
