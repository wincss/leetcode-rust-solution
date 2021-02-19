use crate::*;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = HashMap::new();
        let mut degree = 0;
        for &v in nums.iter() {
            let c = count.entry(v).or_insert(0);
            *c += 1;
            degree = degree.max(*c);
        }
        let mut candidate = HashSet::new();
        let mut count = HashMap::new();
        let mut result = n;
        let mut right = 0;
        for left in 0..n {
            if left > 0 {
                count.entry(nums[left - 1]).and_modify(|v| *v -= 1);
                candidate.remove(&nums[left - 1]);
            }
            while right < n && candidate.is_empty() {
                let v = count.entry(nums[right]).or_insert(0);
                *v += 1;
                if *v == degree {
                    candidate.insert(nums[right]);
                }
                right += 1;
            }
            if !candidate.is_empty() {
                result = result.min(right - left);
            }
        }
        result as i32
    }
}
