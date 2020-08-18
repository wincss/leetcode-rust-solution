use crate::*;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut dp = vec![vec![true; n]; 3];
        let mut ans = n;
        for length in 2..=n {
            let index = length % 3;
            for start in 0..n + 1 - length {
                dp[index][start] =
                    s[start] == s[start + length - 1] && dp[(length - 2) % 3][start + 1];
                if dp[index][start] {
                    ans += 1;
                }
            }
        }
        ans as i32
    }
}
