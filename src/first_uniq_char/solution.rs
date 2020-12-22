use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0) += 1
        }
        for (i, c) in s.chars().enumerate() {
            if h[&c] == 1 {
                return i as i32;
            }
        }
        -1
    }
}
