use crate::*;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for (i, v) in nums.iter().enumerate() {
            result ^= i as i32 + 1;
            result ^= v;
        }
        result
    }
}
