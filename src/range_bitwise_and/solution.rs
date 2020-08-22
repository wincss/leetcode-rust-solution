use crate::*;

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut u = m & n;
        let mut b = 1 << 30;
        while b > 0 {
            if (u & b > 0) && (n - m >= b) {
                u ^= b;
            }
            b >>= 1;
        }
        u
    }
}
