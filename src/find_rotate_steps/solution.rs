use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let m = key.len();
        let ring: Vec<char> = ring.chars().collect();
        let key: Vec<char> = key.chars().collect();

        let mut ring_char = HashMap::new();
        for (i, &v) in ring.iter().enumerate() {
            ring_char.entry(v).or_insert(vec![]).push(i);
        }
        let mut dp = vec![vec![0; n]; 2];
        for i in (0..m).rev() {
            for j in 0..n {
                dp[i & 1][j] = if ring[j] == key[i] {
                    dp[i & 1 ^ 1][j]
                } else {
                    ring_char[&key[i]]
                        .iter()
                        .map(|&k| {
                            dp[i & 1 ^ 1][k]
                                + std::cmp::min((k + n - j) % n, (j + n - k) % n) as i32
                        })
                        .min()
                        .unwrap()
                } + 1
            }
        }
        dp[0][0]
    }
}
