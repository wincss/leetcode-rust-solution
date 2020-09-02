use crate::*;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = piles.clone();
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                dp[j] = std::cmp::max(piles[i] - dp[j], piles[j] - dp[j - 1]);
            }
        }
        dp[n - 1] > 0
    }
}
