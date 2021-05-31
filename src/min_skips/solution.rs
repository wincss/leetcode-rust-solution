use crate::*;

impl Solution {
    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let speed = speed as i64;
        let hours_before = hours_before as i64;
        let n = dist.len();
        let mut dp = vec![std::i64::MAX; n];
        dp[0] = dist[0] as i64;
        // println!("{:?}", dp);
        for i in 1..n {
            for j in (0..=i).rev() {
                if j < i {
                    dp[j] = (dp[j] + speed - 1) / speed * speed + dist[i] as i64;
                }
                if j > 0 {
                    dp[j] = dp[j].min(dp[j - 1] + dist[i] as i64)
                }
            }
            // println!("{:?}", dp);
        }
        for i in 0..n {
            if dp[i] <= hours_before * speed {
                return i as i32;
            }
        }
        -1
    }
}
