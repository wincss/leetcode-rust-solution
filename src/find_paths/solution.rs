use crate::*;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let m = m as usize;
        let n = n as usize;
        let mut result = 0;
        let mut dp = vec![vec![0; n]; m];
        fn calc(dp: &Vec<Vec<i64>>, m: usize, n: usize, result: &mut i64) {
            for i in 0..m {
                *result += dp[i][0];
                *result += dp[i][n - 1];
            }
            for j in 0..n {
                *result += dp[0][j];
                *result += dp[m - 1][j];
            }
            *result %= MOD;
        };
        dp[start_row as usize][start_column as usize] = 1;
        for _ in 0..max_move {
            calc(&dp, m, n, &mut result);
            let mut ndp = vec![vec![0; n]; m];
            for i in 0..m {
                for j in 0..n {
                    if i > 0 {
                        ndp[i][j] += dp[i - 1][j];
                    }
                    if i < m - 1 {
                        ndp[i][j] += dp[i + 1][j];
                    }
                    if j > 0 {
                        ndp[i][j] += dp[i][j - 1];
                    }
                    if j < n - 1 {
                        ndp[i][j] += dp[i][j + 1];
                    }
                    ndp[i][j] %= MOD;
                }
            }
            dp = ndp;
            // println!("{:?} {}", dp, result);
        }
        (result % MOD) as i32
    }
}
