use crate::*;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];
        result.extend(encoded.into_iter().scan(first, |a, b| {
            *a ^= b;
            Some(*a)
        }));
        result
    }
}
