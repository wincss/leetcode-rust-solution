use crate::*;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        for i in 1..=n / 2 {
            if n % i == 0 && s == s[..i].repeat(n / i) {
                return true;
            }
        }
        false
    }
}
