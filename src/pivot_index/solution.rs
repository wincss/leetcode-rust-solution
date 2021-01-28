use crate::*;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for &v in nums.iter() {
            sum += v;
        }
        let mut left = 0;
        for (i, &v) in nums.iter().enumerate() {
            if left == sum - left - v {
                return i as i32;
            }
            left += v;
        }
        -1
    }
}
