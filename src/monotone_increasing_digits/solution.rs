use crate::*;

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut ones = 111111111;
        let mut result = 0;
        for _ in 0..9 {
            while result + ones > n {
                ones /= 10;
            }
            result += ones;
        }
        result
    }
}
