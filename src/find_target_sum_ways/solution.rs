use crate::*;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let sum = nums.iter().fold(0, |s, &v| s + v);
        let negsum2 = sum - target;
        if negsum2 < 0 || negsum2 % 2 != 0 {
            return 0;
        }
        let negsum = (negsum2 / 2) as usize;
        let mut dp = vec![0; negsum + 1];
        dp[0] = 1;
        for i in 0..n {
            let num = nums[i] as usize;
            for j in (0..=negsum).rev() {
                if j >= num {
                    dp[j] += dp[j - num];
                }
            }
        }
        dp[negsum]
    }
}
