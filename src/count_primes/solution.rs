use crate::*;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut prime = vec![true; n];
        let mut count = 0;
        for i in 2..n {
            if !prime[i] {
                continue;
            }
            count += 1;
            for j in 2..=(n - 1) / i {
                prime[i * j] = false;
            }
        }
        count
    }
}
