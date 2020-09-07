use crate::*;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut result = 0;
        for i in s.chars() {
            result = 26 * result + i as u8 as i32 - 64;
        }
        result
    }
}
