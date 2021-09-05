use crate::*;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target)
            .ok()
            .map(|x| x as i32)
            .unwrap_or(-1)
    }
}
