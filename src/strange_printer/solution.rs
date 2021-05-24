use crate::*;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            dp[i][i] = 1;
            for j in i + 1..n {
                if s[i] == s[j] {
                    dp[i][j] = dp[i][j - 1];
                } else {
                    dp[i][j] = std::i32::MAX;
                    for k in i..j {
                        dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j]);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}
