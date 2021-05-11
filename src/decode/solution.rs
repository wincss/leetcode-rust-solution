use crate::*;

impl Solution {
    pub fn decode_1720(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];
        result.extend(encoded.into_iter().scan(first, |a, b| {
            *a ^= b;
            Some(*a)
        }));
        result
    }
    pub fn decode_1734(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let mut first = 0;
        for i in 1..=n {
            first ^= i as i32;
            if i < n - 1 && i & 1 == 1 {
                first ^= encoded[i];
            }
        }
        Self::decode_1720(encoded, first)
    }
}
