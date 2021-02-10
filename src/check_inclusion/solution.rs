use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        Self::check_inclusion_by_sliding_window(s1, s2)
    }

    pub fn check_inclusion_by_sliding_window(s1: String, s2: String) -> bool {
        let mut c1 = [0; 26];
        for c in s1.chars() {
            c1[c as u8 as usize - 97] += 1;
        }
        fn cmp(c1: &[i32; 26], c2: &[i32; 26]) -> i32 {
            for i in 0..26 {
                if c1[i] < c2[i] {
                    return -1;
                }
                if c1[i] > c2[i] {
                    return 1;
                }
            }
            0
        }
        let s2: Vec<char> = s2.chars().collect();
        let n = s2.len();
        let mut c2 = [0; 26];
        let mut right = 0;
        for i in 0..n {
            if i > 0 {
                c2[s2[i - 1] as u8 as usize - 97] -= 1;
            }
            while cmp(&c1, &c2) == 1 && right < n {
                c2[s2[right] as u8 as usize - 97] += 1;
                right += 1;
            }
            if cmp(&c1, &c2) == 0 {
                return true;
            }
        }
        false
    }

    pub fn check_inclusion_by_hashset(s1: String, s2: String) -> bool {
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
