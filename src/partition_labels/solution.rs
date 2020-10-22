use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_ch = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            last_ch.insert(c, i);
        }
        let mut result = vec![];
        let mut part_start = 0;
        let mut part_end = 0;
        for (i, c) in s.chars().enumerate() {
            if i > part_end {
                result.push(i as i32 - part_start as i32);
                part_start = i;
            }
            if last_ch[&c] > part_end {
                part_end = last_ch[&c];
            }
        }
        result.push(s.len() as i32 - part_start as i32);
        result
    }
}
