use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let ring: Vec<char> = ring.chars().collect();
        let mut ring_char = HashMap::new();
        for (i, &v) in ring.iter().enumerate() {
            ring_char.entry(v).or_insert(vec![]).push(i);
        }

        let mut dp = vec![vec![0; n]; 2];
        for (i, c) in key.char_indices().rev() {
            for j in 0..n {
                if ring[j] == c {
                    dp[i & 1][j] = 1 + dp[i & 1 ^ 1][j];
                    continue;
                }
                dp[i & 1][j] = 1 + ring_char[&c]
                    .iter()
                    .map(|&k| {
                        dp[i & 1 ^ 1][k] + std::cmp::min((k + n - j) % n, (j + n - k) % n) as i32
                    })
                    .min()
                    .unwrap();
            }
        }
        dp[0][0]
    }
}
