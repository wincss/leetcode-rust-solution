use crate::*;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..n / 2 {
            let tmp = s[i];
            s[i] = s[n - 1 - i];
            s[n - 1 - i] = tmp;
        }
    }
}
