use crate::*;

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let n = cost.len();
        let m = cost[0].len();
        let mut cost_sum = vec![vec![0; 1 << m]; n];
        for i in 0..n {
            for j in 0..1 << m {
                for k in 0..m {
                    if j & (1 << k) > 0 {
                        cost_sum[i][j] += cost[i][k];
                    }
                }
            }
        }
        // println!("{:?}", cost_sum);
        let mut dp = vec![vec![]; 2];
        dp[0] = cost_sum[0].clone();
        for i in 1..n {
            dp[i % 2] = vec![std::i32::MAX; 1 << m];
            for j in 1..1 << m {
                for k in 1..1 << m {
                    dp[i % 2][j | k] =
                        std::cmp::min(dp[i % 2][j | k], dp[1 - i % 2][k] + cost_sum[i][j]);
                }
            }
            // println!("{}: {:?}", i, dp[i % 2]);
        }
        dp[(n - 1) % 2][(1 << m) - 1]
    }
}
