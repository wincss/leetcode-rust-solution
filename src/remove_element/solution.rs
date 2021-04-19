use crate::*;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;
        let n = nums.len();
        for i in 0..n {
            if val == nums[i] {
                continue;
            }
            nums[idx] = nums[i];
            idx += 1;
        }
        idx as i32
    }
}
