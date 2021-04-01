use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, _: i32) -> i32 {
        let s: Vec<usize> = s.chars().map(|v| v as u8 as usize - 97).collect();
        let n = s.len();
        let mut max_freq = 0;
        let size = min_size as usize;
        let mut h = HashMap::new();
        let mut count = [0; 26];
        let mut letters = 0;

        for i in 0..n {
            if i >= size {
                count[s[i - size]] -= 1;
                if count[s[i - size]] == 0 {
                    letters -= 1;
                }
            }
            count[s[i]] += 1;
            if count[s[i]] == 1 {
                letters += 1;
            }
            if i >= size - 1 && letters <= max_letters {
                let v = h.entry(&s[i + 1 - size..=i]).or_insert(0);
                *v += 1;
                max_freq = max_freq.max(*v);
            }
        }
        max_freq
    }
}
