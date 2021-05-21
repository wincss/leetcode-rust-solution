use crate::*;

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        nums.len() & 1 == 0 || nums.into_iter().fold(0, |a, b| a ^ b) == 0
    }
}
