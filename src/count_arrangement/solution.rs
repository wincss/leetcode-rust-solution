use crate::*;

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let c = 1 << n;
        let mut dp = vec![0; c];
        dp[0] = 1;
        for i in 1..c {
            for j in 0..n {
                if i & (1 << j) == 0 {
                    continue;
                }
                if (j + 1) % i.count_ones() as usize == 0 || i.count_ones() as usize % (j + 1) == 0
                {
                    dp[i] += dp[i ^ (1 << j)];
                }
            }
        }

        dp[c - 1]
    }
}
