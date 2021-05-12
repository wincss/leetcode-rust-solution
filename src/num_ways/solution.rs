use crate::*;

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        const MOD: u32 = 1000000007;
        let n = arr_len.min(steps) as usize;
        let steps = steps as usize;
        let mut dp = vec![vec![0_u32; n]; 2];
        dp[0][0] = 1;
        for i in 1..=steps {
            let key = i & 1;
            let last = 1 - key;
            for i in 0..n {
                dp[key][i] = dp[last][i];
                if i > 0 {
                    dp[key][i] += dp[last][i - 1];
                }
                if i < n - 1 {
                    dp[key][i] += dp[last][i + 1];
                }
                dp[key][i] %= MOD;
            }
        }
        dp[steps & 1][0] as i32
    }
}
