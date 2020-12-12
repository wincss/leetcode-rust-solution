use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut h = HashSet::new();
        for i in nums.into_iter() {
            if !h.insert(i) {
                return true;
            }
        }
        false
    }
}
