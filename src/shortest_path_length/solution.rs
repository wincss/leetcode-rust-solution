use crate::*;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut dp = vec![vec![std::i32::MAX; n]; 1 << n];
        for i in 0..n {
            dp[1 << i][i] = 0;
            // dp[0][i] = 0;
        }

        for i in 2..(1 << n) {
            for _ in 0..2 {
                for j in 0..n {
                    if i & (1 << j) == 0 {
                        continue;
                    }
                    for &k in graph[j].iter() {
                        let k = k as usize;
                        dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(1));
                        dp[i][j] = dp[i][j].min(dp[i ^ (1 << j)][k].saturating_add(1));
                    }
                }
            }
        }
        // for i in 0..(1 << n) {
        //     println!("{}: {:?}", i, dp[i]);
        // }
        *dp[(1 << n) - 1].iter().min().unwrap()
    }
}
