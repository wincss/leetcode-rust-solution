use crate::*;
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in (1..n).rev() {
            for j in i + 1..=n {
                dp[i][j] = (i..j)
                    .map(|k| k + dp[i][k - 1].max(dp[k + 1][j]))
                    .min()
                    .unwrap();
            }
        }
        dp[1][n] as i32
    }
}
