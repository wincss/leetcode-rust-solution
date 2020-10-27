use crate::*;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut d = HashMap::new();
        for i in arr {
            let v = d.entry(i).or_insert(0);
            *v += 1;
        }
        let n = d.len();
        let s: HashSet<i32> = d.values().map(|v| *v).collect();
        n == s.len()
    }
}
