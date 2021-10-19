use crate::*;
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let min = *nums.iter().min().unwrap() as i64;
        (sum - min * nums.len() as i64) as i32
    }
}
