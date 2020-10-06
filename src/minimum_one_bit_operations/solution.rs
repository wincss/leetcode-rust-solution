use crate::*;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;
        while n > 0 {
            result ^= n;
            n >>= 1;
        }
        result
    }
}
