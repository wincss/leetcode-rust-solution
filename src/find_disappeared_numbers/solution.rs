use crate::*;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut slot = vec![false; n];
        for i in nums {
            slot[i as usize - 1] = true;
        }
        let mut result = vec![];
        for (i, v) in slot.into_iter().enumerate() {
            if !v {
                result.push(i as i32 + 1);
            }
        }
        result
    }
}
