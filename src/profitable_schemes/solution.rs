use crate::*;

impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let n = n as usize;
        let m = group.len();
        let sum = profit.iter().fold(0, |s, &v| s + v) as usize;
        let min_profit = min_profit as usize;
        if min_profit > sum {
            return 0;
        }
        let mut dp = vec![vec![0_i64; min_profit + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = 1;
        }
        for i in 0..m {
            for j in (0..=n).rev() {
                for k in (0..=min_profit).rev() {
                    if j >= group[i] as usize {
                        dp[j][k] += dp[j - group[i] as usize][k.saturating_sub(profit[i] as usize)];
                        dp[j][k] %= MOD;
                    }
                }
            }
        }
        dp[n][min_profit] as i32
    }
}
