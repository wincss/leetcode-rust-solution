use crate::*;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut s = 0;
        for (idx, v) in arr.into_iter().enumerate() {
            s += v * ((idx + 1) / 2 * ((n - idx) / 2) + (idx / 2 + 1) * ((n - idx + 1) / 2)) as i32;
        }
        s
    }
}
