use crate::*;

use std::collections::BinaryHeap;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut h = BinaryHeap::from(stones);
        while h.len() > 1 {
            let a = h.pop().unwrap();
            let b = h.pop().unwrap();
            if a != b {
                h.push((a - b).abs());
            }
        }
        h.pop().unwrap_or(0)
    }
}
