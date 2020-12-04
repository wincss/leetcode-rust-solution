use crate::*;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut invalid = 0;
        let mut h = HashMap::new();
        for i in nums.into_iter() {
            let current_length =
                if let Some(Reverse(v)) = h.entry(i - 1).or_insert(BinaryHeap::new()).pop() {
                    if v < 3 {
                        invalid -= 1;
                    }
                    v
                } else {
                    0
                };
            if current_length + 1 < 3 {
                invalid += 1;
            }
            h.entry(i)
                .or_insert(BinaryHeap::new())
                .push(Reverse(current_length + 1));
            // println!("{:?}, {}", h, invalid);
        }
        invalid == 0
    }
}
