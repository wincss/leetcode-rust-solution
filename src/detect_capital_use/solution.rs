use crate::*;
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        // 0 unknown, 1 first cap, 2 all lowercase, 3 full uppercase
        let mut status = 0;
        for i in word.chars() {
            if i.is_ascii_lowercase() {
                status = match status {
                    3 => return false,
                    _ => 2,
                }
            } else {
                status = match status {
                    0 => 1,
                    2 => return false,
                    _ => 3,
                }
            }
        }
        true
    }
}
