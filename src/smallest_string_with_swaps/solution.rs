use crate::*;

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut s: Vec<_> = s.chars().collect();
        let n = s.len();
        let mut h = HashMap::new();
        for pair in pairs {
            h.entry(pair[0] as usize)
                .or_insert(vec![])
                .push(pair[1] as usize);
            h.entry(pair[1] as usize)
                .or_insert(vec![])
                .push(pair[0] as usize);
        }
        let mut visited = HashSet::new();
        let mut groups = vec![];
        let mut q = VecDeque::new();
        for i in 0..n {
            if !h.contains_key(&i) || !visited.insert(i) {
                continue;
            }
            let g = groups.len();
            q.push_back(i);
            groups.push(vec![i]);
            while let Some(i) = q.pop_front() {
                for &j in h[&i].iter() {
                    if visited.insert(j) {
                        q.push_back(j);
                        groups[g].push(j);
                    }
                }
            }
        }
        for mut group in groups {
            let mut c: Vec<_> = group.iter().map(|&idx| s[idx]).collect();
            group.sort();
            c.sort();
            for (idx, ch) in group.into_iter().zip(c.into_iter()) {
                s[idx] = ch;
            }
        }
        s.iter().collect()
    }
}
