use crate::*;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut i = 2;
        let mut n = n;
        let mut s = 0;
        while n > 1 {
            if n % i == 0 {
                s += i;
                n /= i;
            } else {
                i += 1;
            }
        }
        s
    }
}
