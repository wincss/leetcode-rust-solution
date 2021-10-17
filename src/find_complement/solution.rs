use crate::*;
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        num ^ ((1 << (32 - num.leading_zeros())) - 1)
    }
}
