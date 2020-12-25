use crate::*;

impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        let mut cuboids = cuboids;
        for i in cuboids.iter_mut() {
            i.sort();
        }
        cuboids.sort();
        let n = cuboids.len();
        let mut dp = vec![0; n];
        let mut result = 0;
        for i in (0..n).rev() {
            dp[i] = (i + 1..n)
                .filter(|&v| {
                    cuboids[v][0] >= cuboids[i][0]
                        && cuboids[v][1] >= cuboids[i][1]
                        && cuboids[v][2] >= cuboids[i][2]
                })
                .map(|v| dp[v])
                .max()
                .unwrap_or(0)
                + cuboids[i][2];
            result = std::cmp::max(result, dp[i]);
        }
        result
    }
}
