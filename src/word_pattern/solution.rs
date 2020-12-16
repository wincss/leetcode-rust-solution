use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut c2w = HashMap::new();
        let mut w2c = HashMap::new();
        let wordnum = s.split(' ').count();
        if pattern.len() != wordnum {
            return false;
        }
        for (c, w) in pattern.chars().zip(s.split(' ')) {
            if let Some(v) = c2w.get(&c) {
                if *v != w {
                    return false;
                }
            }
            if let Some(v) = w2c.get(&w) {
                if *v != c {
                    return false;
                }
            }
            c2w.insert(c, w);
            w2c.insert(w, c);
        }
        true
    }
}
