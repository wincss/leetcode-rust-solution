use crate::*;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_ascii_whitespace().fold(0, |a, _| a + 1)
    }
}
