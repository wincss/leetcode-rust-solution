use crate::*;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let m = primes.len();
        let mut idx = vec![0; m];
        let mut result = vec![1];
        for _ in 1..n {
            let min = (0..m).map(|v| primes[v] * result[idx[v]]).min().unwrap();
            result.push(min);
            for j in 0..m {
                if primes[j] * result[idx[j]] == min {
                    idx[j] += 1;
                }
            }
        }
        result[n - 1]
    }
}
