use crate::*;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![-1; n];
        let mut s = vec![];
        for i in 0..2 * n {
            while !s.is_empty() && nums[*s.last().unwrap()] < nums[i % n] {
                let idx = s.pop().unwrap();
                result[idx] = nums[i % n];
            }
            s.push(i % n);
        }
        result
    }
}
