use crate::*;

impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fib = vec![1, 1];
        let mut n = 1;
        while fib[n] < k {
            fib.push(fib[n] + fib[n - 1]);
            n += 1;
        }
        let mut k = k;
        let mut step = 0;
        while k > 0 {
            if fib[n] <= k {
                k -= fib[n];
                step += 1;
            } else {
                n -= 1;
            }
        }
        step
    }
}
