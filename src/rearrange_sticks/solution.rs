use crate::*;

impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        const MOD: usize = 1000000007;
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; 2];
        dp[1][1] = 1;
        for i in 2..=n {
            for j in 1..=k {
                dp[i & 1][j] = (dp[i & 1 ^ 1][j - 1] + dp[i & 1 ^ 1][j] * (i - 1)) % MOD;
            }
        }
        dp[n & 1][k] as i32
    }
}
