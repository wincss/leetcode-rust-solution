use crate::*;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![std::usize::MAX; n + 1];
        dp[1] = 0;
        for i in 2..=n {
            for j in 1..=(i as f64).sqrt() as usize {
                if i % j == 0 {
                    dp[i] = dp[i].min(dp[j].saturating_add(i / j));
                    dp[i] = dp[i].min(dp[i / j].saturating_add(j));
                }
            }
        }
        dp[n] as i32
    }
}
