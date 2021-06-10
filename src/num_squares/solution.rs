use crate::*;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![n as i32; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            for j in 1..=i {
                if j * j > i {
                    break;
                }
                dp[i] = dp[i].min(1 + dp[i - j * j]);
            }
        }
        // println!("{:?}", dp);
        dp[n]
    }
}
