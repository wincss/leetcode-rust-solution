use crate::*;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 1 {
            return false;
        }
        let mut n = n;
        while n > 1 {
            if n % 3 != 0 {
                return false;
            }
            n /= 3;
        }
        true
    }
}
