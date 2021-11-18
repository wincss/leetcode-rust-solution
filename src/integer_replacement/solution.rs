use crate::*;
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut step = 0;
        let mut n = n;
        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
                step += 1;
            } else if n % 4 == 1 || n == 3 {
                n /= 2;
                step += 2;
            } else {
                n = n / 2 + 1;
                step += 2;
            }
        }
        step
    }
}
