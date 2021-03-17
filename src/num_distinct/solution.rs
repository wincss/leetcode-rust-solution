use crate::*;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let n = s.len();
        let m = t.len();
        let mut dp = vec![0; m];
        for i in (0..n).rev() {
            let mut last = 1;
            for j in (0..m).rev() {
                let tmp = dp[j];
                if s[i] == t[j] {
                    dp[j] += last;
                }
                last = tmp;
            }
            // println!("{:?}", dp);
        }
        dp[0]

        // let mut dp = vec![vec![0; m + 1]; n + 1];
        // for i in 0..=n {
        //     dp[i][m] = 1;
        // }
        // for i in 0..m {
        //     dp[n][i] = 0;
        // }
        // for i in (0..n).rev() {
        //     for j in (0..m).rev() {
        //         dp[i][j] = dp[i + 1][j];
        //         if s[i] == t[j] {
        //             dp[i][j] += dp[i + 1][j + 1];
        //         }
        //     }
        // }
        // dp[0][0]
    }
}
