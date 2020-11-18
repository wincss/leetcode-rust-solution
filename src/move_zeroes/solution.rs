use crate::*;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut non_zero = 0;
        for i in 0..n {
            if nums[i] != 0 {
                nums[non_zero] = nums[i];
                non_zero += 1;
            }
        }
        for i in non_zero..n {
            nums[i] = 0;
        }
    }
}
