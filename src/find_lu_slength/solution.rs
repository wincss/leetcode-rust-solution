use crate::*;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        let al = a.len();
        let bl = b.len();
        if al == bl && a == b {
            -1
        } else {
            al.max(bl) as i32
        }
    }
}
