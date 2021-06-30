use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn num_ways_lcp_07(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let mut path = vec![HashSet::new(); n];
        for rel in relation.into_iter() {
            path[rel[0] as usize].insert(rel[1] as usize);
        }
        let mut msg = vec![0; n];
        msg[0] = 1;
        for _ in 0..k {
            let mut new_msg = vec![0; n];
            for from in 0..n {
                for &to in path[from].iter() {
                    new_msg[to] += msg[from];
                }
            }
            msg = new_msg;
        }
        msg[n - 1]
    }

    pub fn num_ways_1269(steps: i32, arr_len: i32) -> i32 {
        const MOD: u32 = 1000000007;
        let n = arr_len.min(steps) as usize;
        let steps = steps as usize;
        let mut dp = vec![vec![0_u32; n]; 2];
        dp[0][0] = 1;
        for i in 1..=steps {
            let key = i & 1;
            let last = 1 - key;
            for i in 0..n {
                dp[key][i] = dp[last][i];
                if i > 0 {
                    dp[key][i] += dp[last][i - 1];
                }
                if i < n - 1 {
                    dp[key][i] += dp[last][i + 1];
                }
                dp[key][i] %= MOD;
            }
        }
        dp[steps & 1][0] as i32
    }
}
