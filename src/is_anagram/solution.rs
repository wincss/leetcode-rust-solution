use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_count = HashMap::new();
        let mut t_count = HashMap::new();
        for i in s.chars() {
            *s_count.entry(i).or_insert(0) += 1;
        }
        for i in t.chars() {
            *t_count.entry(i).or_insert(0) += 1;
        }
        s_count == t_count
    }
}
