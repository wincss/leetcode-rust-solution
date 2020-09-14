use crate::*;

impl Solution {
    pub fn expect_number(scores: Vec<i32>) -> i32 {
        scores
            .into_iter()
            .collect::<std::collections::HashSet<i32>>()
            .len() as i32
    }
}
