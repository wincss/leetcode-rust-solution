use crate::*;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut max = 0;
        for i in nums {
            if i == 1 {
                current += 1;
                max = max.max(current);
            } else {
                current = 0;
            }
        }
        max
    }
}
