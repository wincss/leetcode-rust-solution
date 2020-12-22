use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut h = HashSet::new();
        let mut result = 0;
        let mut current = 0;
        let mut left = 0;
        for &i in nums.iter() {
            while h.contains(&i) {
                current -= nums[left];
                h.remove(&nums[left]);
                left += 1;
            }
            current += i;
            h.insert(i);
            result = std::cmp::max(result, current);
        }
        result
    }
}
