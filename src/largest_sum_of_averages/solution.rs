use crate::*;
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len();
        let mut s = vec![0; n + 1];
        for (i, &v) in nums.iter().enumerate() {
            s[i + 1] = s[i] + v;
        }
        let mut dp = vec![f64::NEG_INFINITY; n + 1];
        dp[n] = 0_f64;
        for _ in 0..k {
            for i in 0..n {
                for j in i + 1..=n {
                    dp[i] = dp[i].max(dp[j] + (s[j] - s[i]) as f64 / ((j - i) as f64));
                }
            }
        }
        dp[0]
    }
}
