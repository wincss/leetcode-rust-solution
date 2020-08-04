use crate::*;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            // println!("nums[{}] = {}", i, v);
            let item = h.entry(v).or_insert(Vec::new());
            (*item).push(i as i32);
        }
        for (&k, v) in h.iter() {
            if k + k == target {
                if v.len() >= 2 {
                    // println!("{:?}", v);
                    return vec![v[0], v[1]];
                }
                continue;
            }
            if let Some(v2) = h.get(&(target - k)) {
                return vec![v[0], v2[0]];
            }
        }
        unreachable!();
    }
}
