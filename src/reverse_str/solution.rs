use crate::*;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut rev = vec![];
        let mut result = String::new();
        for (idx, c) in s.char_indices() {
            if idx % (2 * k) < k {
                rev.push(c);
            } else {
                while let Some(v) = rev.pop() {
                    result.push(v);
                }
                result.push(c);
            }
        }
        while let Some(v) = rev.pop() {
            result.push(v);
        }
        result
    }
}
