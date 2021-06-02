use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut s = HashMap::new();
        s.insert(0, 0);
        let mut last = 0;
        let mut result = 0;
        for (i, v) in nums.into_iter().enumerate() {
            if v == 1 {
                last += 100000;
            } else {
                last += 1;
            }
            last = last % 100001;
            if s.contains_key(&last) {
                result = result.max(i + 1 - s[&last]);
            } else {
                s.insert(last, i + 1);
            }
        }
        result as i32
    }
}
