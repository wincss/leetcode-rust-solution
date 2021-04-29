use crate::*;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut b = vec![0; 32];
        for i in nums.into_iter() {
            for j in 0..32 {
                if i & (1 << j) != 0 {
                    b[j] += 1;
                }
            }
        }
        let mut result = 0;
        for j in 0..32 {
            if b[j] % 3 > 0 {
                result += 1 << j;
            }
        }
        result
    }
}
