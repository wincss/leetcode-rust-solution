use crate::*;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ((((n as i64 * 8 + 1) as f64).sqrt() - 1.0) / 2.0) as i32
    }
}
