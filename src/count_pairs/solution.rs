use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let mut d = HashMap::new();
        for i in deliciousness.into_iter() {
            *d.entry(i).or_insert(0_i64) += 1;
        }
        let mut s = 0;
        for (&k, &v) in d.iter() {
            if k.count_ones() == 1 {
                s += v * (v - 1) / 2;
                s %= MOD;
            }
            for i in 0..=21 {
                let l = (1 << i) - k;
                if l <= k {
                    continue;
                }
                if d.contains_key(&l) {
                    s += v * d[&l];
                    s %= MOD;
                }
            }
        }
        (s % MOD) as i32
    }
}
