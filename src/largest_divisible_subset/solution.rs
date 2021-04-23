use crate::*;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n < 2 {
            return nums;
        }
        let mut nums = nums;
        nums.sort();
        let mut dp = vec![];
        for i in 0..n {
            dp.push(vec![nums[i]]);
            for j in (0..i).rev() {
                if nums[i] % nums[j] == 0 {
                    if dp[j].len() >= dp[i].len() {
                        dp[i] = dp[j].clone();
                        dp[i].push(nums[i]);
                    }
                }
            }
        }
        dp.into_iter().max_by_key(|v| v.len()).unwrap()
    }
}
