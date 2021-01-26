use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut h = HashMap::new();
        for mut item in dominoes.into_iter() {
            item.sort();
            *h.entry(item).or_insert(0) += 1
        }
        let mut result = 0;
        for &v in h.values() {
            result += v * (v - 1) / 2;
        }
        result
    }
}
