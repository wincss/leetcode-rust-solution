use crate::*;

use std::cmp::Reverse;

impl Solution {
    pub fn min_sessions(tasks: Vec<i32>, session_time: i32) -> i32 {
        let n = tasks.len();
        let total = 1 << n;
        let mut dp = vec![(0_i32, 0_i32)];
        for i in 1..total {
            let mut current = (std::i32::MAX, Reverse(0));
            for j in 0..n {
                if i & (1 << j) == 0 {
                    continue;
                }
                let key = i ^ (1 << j);
                if dp[key].1 < tasks[j] {
                    current = current.min((
                        dp[key].0.saturating_add(1),
                        Reverse(session_time - tasks[j]),
                    ));
                } else {
                    current = current.min((dp[key].0, Reverse(dp[key].1 - tasks[j])));
                }
            }
            dp.push((current.0, (current.1).0));
        }
        // println!("{:?}", dp);
        dp[total - 1].0
    }
}
