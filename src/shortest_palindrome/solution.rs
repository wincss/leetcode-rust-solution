use crate::*;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        let mut rev = s.chars().rev().collect::<String>();
        for i in (1..=n).rev() {
            if s[..i] == rev[n - i..] {
                rev.truncate(n - i);
                rev.push_str(&s);
                return rev;
            }
        }
        s
    }
}
