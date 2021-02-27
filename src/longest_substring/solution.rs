use crate::*;

use std::collections::HashMap;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        fn longest_substring_internal(s: &str, k: i32) -> usize {
            let n = s.len();
            let mut h = HashMap::new();
            for c in s.chars() {
                *h.entry(c).or_insert(0) += 1;
            }
            let mut left = 0;
            let mut result = 0;
            for (right, c) in s.char_indices() {
                if h[&c] < k {
                    if right > left {
                        result = result.max(longest_substring_internal(&s[left..right], k));
                    }
                    left = right + 1;
                }
            }
            if left == 0 {
                return n;
            } else if n > left {
                result = result.max(longest_substring_internal(&s[left..n], k));
            }
            result
        }
        longest_substring_internal(&s[..], k) as i32
    }
}
