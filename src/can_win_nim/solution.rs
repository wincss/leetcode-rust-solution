use crate::*;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n & 3 != 0
    }
}