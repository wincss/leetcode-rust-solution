use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for i in nums.into_iter() {
            let v = count.entry(i).or_insert(0);
            *v += 1;
        }
        let mut keys = count.keys().map(|v| *v).collect::<Vec<i32>>();
        keys.sort_unstable_by_key(|v| count[v]);
        Vec::from(&keys[keys.len() - k as usize..])
    }
}
