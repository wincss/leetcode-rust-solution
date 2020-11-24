use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut count = HashMap::new();
        for i in s.chars() {
            *count.entry(i).or_insert(0) += 1;
        }
        let mut keys: Vec<char> = count.keys().map(|v| *v).collect();
        keys.sort();
        let mut result = vec![];
        let mut found = true;
        while found {
            found = false;
            for i in keys.iter() {
                if count[i] > 0 {
                    found = true;
                    result.push(*i);
                    count.entry(*i).and_modify(|v| *v -= 1);
                }
            }
            for i in keys.iter().rev() {
                if count[i] > 0 {
                    found = true;
                    result.push(*i);
                    count.entry(*i).and_modify(|v| *v -= 1);
                }
            }
        }
        result.into_iter().collect()
    }
}
