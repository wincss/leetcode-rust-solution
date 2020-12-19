use crate::*;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut h = HashMap::new();
        for c in s.chars() {
            *h.entry(c).or_insert(0) += 1;
        }
        let mut st = Vec::new();
        let mut used = HashSet::new();
        for c in s.chars() {
            if !used.contains(&c) {
                while let Some(v) = st.last() {
                    if h[v] == 0 {
                        break;
                    }
                    if *v < c {
                        break;
                    }
                    used.remove(v);
                    st.pop();
                }
                st.push(c);
                used.insert(c);
            }
            h.entry(c).and_modify(|v| *v -= 1);
            // println!("st={:?}, used={:?}", st, used);
        }
        st.into_iter().collect()
    }

    pub fn smallest_subsequence(s: String) -> String {
        Solution::remove_duplicate_letters(s)
    }
}
