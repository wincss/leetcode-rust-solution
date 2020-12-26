use crate::*;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        let mut char_in_t = HashSet::new();
        let s: Vec<_> = s.chars().collect();
        let t: Vec<_> = t.chars().collect();
        assert_eq!(s.len(), t.len());
        let n = s.len();
        for i in 0..n {
            if let Some(&v) = map.get(&s[i]) {
                if v != t[i] {
                    return false;
                }
            } else if char_in_t.contains(&t[i]) {
                return false;
            } else {
                map.insert(s[i], t[i]);
                char_in_t.insert(t[i]);
            }
        }
        true
    }
}
