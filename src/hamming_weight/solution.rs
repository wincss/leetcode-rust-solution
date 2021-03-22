use crate::*;

impl Solution {
    // hammingWeight in origin problem
    pub fn hamming_weight(n: u32) -> i32 {
        let mut n = n;
        let mut result = 0;
        while n > 0 {
            n = n & (n - 1);
            result += 1;
        }
        result
    }
}
