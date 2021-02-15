use crate::*;

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums.iter()
            .enumerate()
            .filter_map(|(i, v)| if i % 2 == 0 { Some(*v) } else { None })
            .sum()
    }
}
