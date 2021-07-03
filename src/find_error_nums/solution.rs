use crate::*;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut v = vec![0; n + 1];
        for i in nums.into_iter() {
            v[i as usize] += 1;
        }
        let mut result = vec![0, 0];
        for i in 1..=n {
            match v[i] {
                2 => result[0] = i as i32,
                0 => result[1] = i as i32,
                _ => {}
            }
        }
        result
    }
}
