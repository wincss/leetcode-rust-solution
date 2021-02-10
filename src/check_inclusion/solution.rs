use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut c1 = [0; 26];
        for c in s1.chars() {
            c1[c as u8 as usize - 97] += 1;
        }
        let mut c2 = [0; 26];
        let mut h = HashSet::new();
        for c in s2.chars() {
            c2[c as u8 as usize - 97] += 1;
            h.insert(c2.clone());
        }
        if h.contains(&c1) {
            return true;
        }
        let mut c2 = [0; 26];
        for c in s2.chars() {
            c2[c as u8 as usize - 97] += 1;
            let mut c3 = c2.clone();
            for i in 0..26 {
                c3[i] += c1[i];
            }
            if h.contains(&c3) {
                return true;
            }
        }
        false
    }
}
