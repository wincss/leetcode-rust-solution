use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent<T>(nums: Vec<T>, k: i32) -> Vec<T>
    where
        T: Clone + std::hash::Hash + std::cmp::Eq + std::cmp::PartialOrd,
    {
        let mut count = HashMap::new();
        for i in nums.iter() {
            let v = count.entry(i.clone()).or_insert(0);
            *v += 1;
        }
        let mut keys = count.keys().map(|v| v.clone()).collect::<Vec<T>>();
        keys.sort_unstable_by(|v1, v2| match count[v1].partial_cmp(&count[v2]).unwrap() {
            std::cmp::Ordering::Equal => v1.partial_cmp(&v2).unwrap(),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
        });
        Vec::from(&keys[..k as usize])
    }
}
