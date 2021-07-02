use crate::*;

use std::{cmp::Reverse, collections::HashMap};
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0_usize) += 1;
        }
        let mut h: Vec<_> = h.into_iter().collect();
        h.sort_by_key(|(_, count)| Reverse(*count));
        let mut result = String::new();
        for (c, count) in h.into_iter() {
            result.push_str(&c.to_string().repeat(count));
        }
        result
    }
}
