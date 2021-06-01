use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut s = HashMap::new();
        s.insert(0, 0);
        let mut last = 0;
        for (i, v) in nums.into_iter().enumerate() {
            last = (last + v) % k;
            if s.contains_key(&last) {
                if s[&last] < i {
                    return true;
                }
            } else {
                s.insert(last, i + 1);
            }
        }
        false
    }
}
