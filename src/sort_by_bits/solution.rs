use crate::*;

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        fn nbits(k: i32) -> i32 {
            let mut result = 0;
            let mut k = k;
            while k > 0 {
                k &= k - 1;
                result += 1;
            }
            result
        }
        let mut arr = arr;
        arr.sort_by_key(|&v| (nbits(v), v));
        arr
    }
}
