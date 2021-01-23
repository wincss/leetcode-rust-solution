use crate::*;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut result = 1;
        let mut s = 0;
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                result = std::cmp::max(result, i - s + 1);
            } else {
                s = i;
            }
        }
        result as i32
    }
}
