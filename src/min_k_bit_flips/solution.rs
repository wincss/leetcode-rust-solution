use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;
        let mut result = 0;
        let mut flips = VecDeque::new();
        for (i, v) in a.into_iter().enumerate() {
            while let Some(&f) = flips.front() {
                if f + k <= i {
                    flips.pop_front();
                } else {
                    break;
                }
            }
            if (v ^ flips.len() as i32) & 1 == 0 {
                if i + k > n {
                    return -1;
                }
                flips.push_back(i);
                result += 1;
                // println!("{:?} {}", flips, result);
            }
        }
        result
    }
}
