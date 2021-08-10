use crate::*;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        let mut result = 0;
        let mut diff = None;
        let mut length = 1;
        for i in 1..n {
            if diff.is_none() {
                diff = Some(nums[i] - nums[i - 1]);
                length = 2;
            } else if nums[i] - nums[i - 1] == diff.unwrap() {
                length += 1;
            } else {
                result += (length - 1) * (length - 2) / 2;
                diff = Some(nums[i] - nums[i - 1]);
                length = 2;
            }
        }
        result += (length - 1) * (length - 2) / 2;
        result
    }
}
