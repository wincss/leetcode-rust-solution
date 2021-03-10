use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut palindrome_end = vec![HashSet::new(); n];
        for length in 1..=n {
            for start in 0..=n - length {
                match length {
                    1 => {
                        palindrome_end[start].insert(start + length);
                    }
                    2 if s[start] == s[start + 1] => {
                        palindrome_end[start].insert(start + length);
                    }
                    _ if s[start] == s[start + length - 1]
                        && palindrome_end[start + 1].contains(&(start + length - 1)) =>
                    {
                        palindrome_end[start].insert(start + length);
                    }
                    _ => {}
                }
            }
        }
        let mut dp = vec![std::i32::MAX; n + 1];
        dp[n] = 0;
        for i in (0..n).rev() {
            for &j in palindrome_end[i].iter() {
                dp[i] = dp[i].min(1 + dp[j]);
            }
        }
        dp[0] - 1
    }
}
