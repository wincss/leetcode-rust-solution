use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut d = HashMap::new();
        for w in strs.into_iter() {
            let mut c: Vec<char> = w.clone().chars().collect();
            c.sort();
            d.entry(c).or_insert(vec![]).push(w);
        }
        d.into_iter().map(|(_, v)| v).collect()
    }
}
