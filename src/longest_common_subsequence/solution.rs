use crate::*;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        let l1 = text1.len();
        let l2 = text2.len();
        let mut dp = vec![0; l2];
        for i in 0..l1 {
            let mut last = 0;
            for j in 0..l2 {
                let tmp = dp[j];
                if text1[i] == text2[j] {
                    dp[j] = last + 1;
                } else if j > 0 {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                last = tmp;
            }
            // println!("{:?}", dp);
        }
        dp[l2 - 1]
    }
}
