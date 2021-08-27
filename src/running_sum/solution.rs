use crate::*;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut last = 0;
        let mut nums = nums;
        for i in nums.iter_mut() {
            last += *i;
            *i = last;
        }
        nums
    }
}
