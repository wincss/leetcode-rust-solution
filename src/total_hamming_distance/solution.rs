use crate::*;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut v = vec![0; 32];
        for &i in nums.iter() {
            for j in 0..32 {
                if i & (1 << j) > 0 {
                    v[j] += 1;
                }
            }
        }
        let mut result = 0;
        for &i in nums.iter() {
            for j in 0..32 {
                if i & (1 << j) > 0 {
                    result += n - v[j];
                } else {
                    result += v[j];
                }
            }
        }
        result as i32 / 2
    }
}
