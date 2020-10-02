use crate::*;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h: HashMap<i32, i32> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            if let Some(&sibling) = h.get(&(target - v)) {
                return vec![i as i32, sibling];
            }
            h.insert(v, i as i32);
        }
        unreachable!();
    }
}
