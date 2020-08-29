use crate::*;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ans = vec![];
        for word in s.split_ascii_whitespace() {
            ans.push(word.chars().rev().collect::<String>());
        }
        ans.join(" ")
    }
}
