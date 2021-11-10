use crate::*;
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![0_i64; k + 1]; 2];
        dp[0][0] = 1;
        for i in 1..=n {
            for j in 0..=k {
                dp[i & 1][j] = dp[i & 1 ^ 1][j];
                if j > 0 {
                    dp[i & 1][j] += dp[i & 1][j - 1];
                }
                if j >= i {
                    dp[i & 1][j] -= dp[i & 1 ^ 1][j - i];
                }
                dp[i & 1][j] = dp[i & 1][j].rem_euclid(MOD);
            }
        }
        dp[n & 1][k] as i32
    }
}
