use crate::*;

use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let s = s.as_bytes();
        let mask_array = [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let mut mask = 0;
        let mut ans = 0;
        let mut states = HashMap::new();
        states.insert(0, 0);
        for (i, &c) in s.iter().enumerate() {
            mask ^= 1 << (c - 48);
            for &j in mask_array.iter() {
                match states.get(&(mask ^ j)) {
                    Some(&v) => {
                        ans = cmp::max(ans, i + 1 - v);
                    }
                    None if j == 0 => {
                        states.insert(mask, i + 1);
                    }
                    _ => {}
                }
            }
        }
        ans as i32
    }
}
