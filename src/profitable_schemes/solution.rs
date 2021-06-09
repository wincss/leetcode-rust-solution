use crate::*;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let n = n as usize;
        let m = group.len();
        let sum = profit.iter().fold(0, |s, &v| s + v) as usize;
        if min_profit as usize > sum {
            return 0;
        }
        let mut dp = vec![vec![0_i64; sum + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = 1;
        }
        for i in 0..m {
            for j in (0..=n).rev() {
                for k in (0..=sum).rev() {
                    if j >= group[i] as usize && k >= profit[i] as usize {
                        dp[j][k] += dp[j - group[i] as usize][k - profit[i] as usize];
                        dp[j][k] %= MOD;
                    }
                }
            }
        }
        dp[n][min_profit as usize..]
            .into_iter()
            .fold(0, |s, &v| (s + v) % MOD) as i32
    }
}
