use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut h = HashMap::new();
        for i in 1..31 {
            h.insert((1 << i) - 1, i - 1);
        }
        let mut result = vec![0];
        let mut last = 0;
        for i in 1..=num {
            last = last + 1 - h[&(i ^ (i - 1))];
            result.push(last);
        }
        result
    }
}
