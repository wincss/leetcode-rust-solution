use crate::*;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut is_prime = vec![true; n];
        let mut primes = vec![];
        let mut count = 0;
        for i in 2..n {
            if is_prime[i] {
                count += 1;
                primes.push(i);
            }
            for &j in primes.iter() {
                if i * j >= n {
                    break;
                }
                is_prime[i * j] = false;
                if i % j == 0 {
                    break;
                }
            }
        }
        count
    }
}
