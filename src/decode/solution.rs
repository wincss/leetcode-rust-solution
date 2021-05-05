use crate::*;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];
        let mut last = first;
        for i in encoded {
            last = last ^ i;
            result.push(last);
        }
        result
    }
}
