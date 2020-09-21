use crate::*;

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let n = cost.len();
        let m = cost[0].len();
        let mut dp = vec![vec![]; 2];
        dp[0] = vec![std::i32::MAX; 1 << m];
        dp[0][0] = 0;
        for i in 1..=n {
            dp[i % 2] = vec![std::i32::MAX; 1 << m];
            for j in 1..1 << m {
                for k in 0..m {
                    if (1 << k) & j > 0 {
                        dp[i % 2][j] = std::cmp::min(
                            dp[i % 2][j],
                            dp[1 - i % 2][j].saturating_add(cost[i - 1][k]),
                        );
                        dp[i % 2][j] = std::cmp::min(
                            dp[i % 2][j],
                            dp[i % 2][j ^ (1 << k)].saturating_add(cost[i - 1][k]),
                        );
                        dp[i % 2][j] = std::cmp::min(
                            dp[i % 2][j],
                            dp[1 - i % 2][j ^ (1 << k)].saturating_add(cost[i - 1][k]),
                        );
                    }
                }
            }
            // println!("{}: {:?}", i, dp[i % 2]);
        }
        dp[n % 2][(1 << m) - 1]
    }
}
