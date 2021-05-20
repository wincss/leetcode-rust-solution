use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent<T>(nums: Vec<T>, k: i32) -> Vec<T>
    where
        T: Clone + std::hash::Hash + std::cmp::Eq + std::cmp::PartialOrd,
    {
        let mut count = HashMap::new();
        for i in nums.into_iter() {
            let v = count.entry(i).or_insert(0);
            *v += 1;
        }
        let mut keys = count.keys().collect::<Vec<&T>>();
        keys.sort_unstable_by(|v1, v2| match count[v2].partial_cmp(&count[v1]).unwrap() {
            std::cmp::Ordering::Equal => v1.partial_cmp(v2).unwrap(),
            v => v,
        });
        keys[..k as usize]
            .into_iter()
            .map(|v| (*v).clone())
            .collect()
    }
}
