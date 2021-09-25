use crate::*;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let tmp = a & b;
            a ^= b;
            b = tmp << 1;
        }
        a
    }
}
