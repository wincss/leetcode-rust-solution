use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let k = k as usize;
        let s: Vec<char> = s.chars().collect();
        let mut h = HashMap::new();
        let mut result = 0;
        let mut maxn = 0;
        let mut left = 0;
        for (i, &c) in s.iter().enumerate() {
            let v = h.entry(c).or_insert(0);
            *v += 1;
            maxn = std::cmp::max(maxn, *v);
            while i - left + 1 - maxn > k {
                h.entry(s[left]).and_modify(|v| *v -= 1);
                left += 1;
            }
            result = std::cmp::max(result, i - left + 1);
        }
        result as i32
    }
}
