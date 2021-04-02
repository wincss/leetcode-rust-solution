use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut h = HashMap::new();
        for i in nums {
            *h.entry(i).or_insert(0) += 1;
        }
        let mut sum = 0;
        for (v, count) in h.into_iter() {
            if count == 1 {
                sum += v;
            }
        }
        sum
    }
}
