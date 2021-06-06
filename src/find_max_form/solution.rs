use crate::*;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for s in strs.into_iter() {
            let zeros = s.chars().fold(0, |v, c| if c == '0' { v + 1 } else { v });
            let ones = s.len() - zeros;
            for i in (0..=n).rev() {
                for j in (0..=m).rev() {
                    if i >= ones && j >= zeros {
                        dp[i][j] = dp[i][j].max(dp[i - ones][j - zeros] + 1);
                    }
                }
            }
        }
        dp[n][m]
    }
}
