use crate::*;

use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let sum_a: i32 = a.iter().sum();
        let sum_b: i32 = b.iter().sum();
        let a: HashSet<i32> = a.into_iter().collect();
        let b: HashSet<i32> = b.into_iter().collect();
        for &i in a.iter() {
            let diff = sum_b - sum_a + 2 * i;
            if diff & 1 == 0 && b.contains(&(diff / 2)) {
                return vec![i, diff / 2];
            }
        }
        unreachable!();
    }
}
