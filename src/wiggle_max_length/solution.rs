use crate::*;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }
        let mut result = n as i32;
        let mut last2 = nums[0];
        let mut last1 = nums[1];
        let mut p = 1;
        while last1 == last2 && p < n {
            result -= 1;
            p += 1;
            if p == n {
                return 1;
            }
            last1 = nums[p];
        }
        for i in p + 1..n {
            if (nums[i] - last1) * (last1 - last2) >= 0 {
                result -= 1;
                if last1 > last2 {
                    last1 = std::cmp::max(last1, nums[i]);
                } else {
                    last1 = std::cmp::min(last1, nums[i]);
                }
            } else {
                last2 = last1;
                last1 = nums[i];
            }
        }
        result
    }
}
