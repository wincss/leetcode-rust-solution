use crate::*;

impl Solution {
    pub fn ways_to_step(n: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let mut l1 = 0_i64;
        let mut l2 = 0_i64;
        let mut l3 = 1_i64;
        let mut t: i64;
        for _ in 1..=n {
            t = (l1 + l2 + l3) % MOD;
            l1 = l2;
            l2 = l3;
            l3 = t;
        }
        l3 as i32
    }
}
