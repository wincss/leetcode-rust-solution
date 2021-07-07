use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let mut h = HashMap::new();
        let mut s = 0;
        let mut result = 0;
        h.insert(0, 1);
        for i in nums.into_iter() {
            s += i;
            result += *h.get(&(s - goal)).unwrap_or(&0);
            *h.entry(s).or_insert(0) += 1;
        }
        result
    }
}
