use crate::*;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp = vec![0; n];
        for i in 0..n {
            let mut last = 0;
            for j in 0..n {
                let tmp = dp[j];
                if s[i] == s[n - 1 - j] {
                    dp[j] = last + 1;
                } else if j > 0 {
                    dp[j] = dp[j].max(dp[j - 1]);
                }
                last = tmp;
            }
            // println!("{:?}", dp);
        }
        dp[n - 1]
    }
}
