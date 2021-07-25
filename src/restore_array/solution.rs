use crate::*;
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut link = HashMap::new();
        for item in adjacent_pairs.into_iter() {
            link.entry(item[0])
                .or_insert(HashSet::new())
                .insert(item[1]);
            link.entry(item[1])
                .or_insert(HashSet::new())
                .insert(item[0]);
        }
        let mut start = 0;
        for (key, value) in link.iter() {
            if value.len() == 1 {
                start = *key;
                break;
            }
        }
        let mut result = vec![start];
        while !link.is_empty() {
            let mut next = 0;
            for &i in link[&start].iter() {
                next = i;
                break;
            }
            link.remove(&start);
            if let Some(v) = link.get_mut(&next) {
                v.remove(&start);
                result.push(next);
            }
            start = next;
        }
        result
    }
}
