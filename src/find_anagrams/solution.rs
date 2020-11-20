use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let ls = s.len();
        let lp = p.len();

        let mut p_count = HashMap::new();
        for i in p.chars() {
            *p_count.entry(i).or_insert(0) += 1;
        }

        let mut result = vec![];
        let s: Vec<char> = s.chars().collect();
        let mut s_count = HashMap::new();
        for i in 0..ls {
            *s_count.entry(s[i]).or_insert(0) += 1;
            if i >= lp {
                let v = s_count[&s[i - lp]] - 1;
                if v > 0 {
                    s_count.insert(s[i - lp], v);
                } else {
                    s_count.remove(&s[i - lp]);
                }
            }
            // println!("i={},lp={},s={:?},p={:?}", i, lp, s_count, p_count);

            if s_count == p_count {
                result.push((i + 1 - lp) as i32);
            }
        }
        result
    }
}
