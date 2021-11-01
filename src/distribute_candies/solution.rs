use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut s = HashSet::new();
        let n = candy_type.len();
        for t in candy_type.into_iter() {
            s.insert(t);
        }
        (if s.len() > n / 2 { n / 2 } else { s.len() }) as i32
    }
}
