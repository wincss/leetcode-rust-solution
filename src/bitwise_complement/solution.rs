use crate::*;
impl Solution {
    /*
     * See find_complement (476)
     * n can be 0 here.
     */
    pub fn bitwise_complement(n: i32) -> i32 {
        n ^ ((1 << 1.max(32 - n.leading_zeros())) - 1)
    }
}
