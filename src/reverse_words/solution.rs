use crate::*;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|v| v.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
