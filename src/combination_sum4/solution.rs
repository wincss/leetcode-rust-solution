use crate::*;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 0..=target as usize {
            for &j in nums.iter() {
                if i as i32 + j > target {
                    continue;
                }
                dp[i + j as usize] += dp[i];
            }
        }
        dp[target as usize]
    }
}
