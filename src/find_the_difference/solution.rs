use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *h.entry(c).or_insert(0) -= 1;
            if h[&c] < 0 {
                return c;
            }
        }
        unreachable!();
    }
}
