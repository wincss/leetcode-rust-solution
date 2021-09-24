use crate::*;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut dp = vec![0; word2.len()];
        for i in 0..word1.len() {
            let mut last = 0;
            for j in 0..word2.len() {
                let tmp = dp[j];
                if word1[i] == word2[j] {
                    dp[j] = dp[j].max(last + 1);
                } else if j > 0 {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                last = tmp;
            }
        }
        (word1.len() + word2.len()) as i32 - dp[word2.len() - 1] * 2
    }
}
