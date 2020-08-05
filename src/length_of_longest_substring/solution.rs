use crate::*;

use std::cmp;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut count: HashSet<u8> = HashSet::new();
        let mut ans = 0;
        let mut l = 0;
        let s = s.as_bytes();
        for (r, &c) in s.iter().enumerate() {
            if count.contains(&c) {
                while s[l] != c {
                    count.remove(&s[l]);
                    l += 1;
                }
                l += 1;
            } else {
                count.insert(c);
            }
            ans = cmp::max(ans, r - l + 1);
        }
        ans as i32
    }
}
