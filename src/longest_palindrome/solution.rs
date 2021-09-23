use crate::*;

impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let mut s = vec![];
        s.extend(word1.chars());
        s.extend(word2.chars());
        let n = s.len();
        let l1 = word1.len();

        let mut result = 0;

        let mut dp = vec![0; n];
        for i in (0..n).rev() {
            let mut last = dp[i];
            dp[i] = 1;
            for j in (i + 1)..n {
                let tmp = dp[j];
                if s[i] == s[j] {
                    dp[j] = last + 2;
                    if dp[j] > result && i < l1 && j >= l1 {
                        result = dp[j]
                    }
                } else if j > 0 {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                last = tmp;
            }
            // println!("{:?}", dp);
        }
        result
    }
}
