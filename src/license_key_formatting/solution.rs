use crate::*;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let k = k as usize;
        let mut c = 0;
        for i in s.chars() {
            if i != '-' {
                c += 1;
            }
        }
        if c == 0 {
            return String::new();
        }
        let dash = (c - 1) / k;
        let first = c % k;
        let mut result = String::with_capacity(c + dash);
        let mut length = 0;
        for i in s.chars() {
            if i != '-' {
                if length > 0 && length >= first && (length - first) % k == 0 {
                    result.push('-');
                }
                result.push(i.to_ascii_uppercase());
                length += 1;
            }
        }
        result
    }
}
