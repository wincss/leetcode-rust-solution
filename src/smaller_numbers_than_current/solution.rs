use crate::*;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums: Vec<(i32, usize)> =
            nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        nums.sort();
        let mut result = vec![0; n];
        let mut last = -1;
        let mut last_pos = 0;
        for (i, (v, j)) in nums.into_iter().enumerate() {
            if v == last {
                result[j] = last_pos;
            } else {
                result[j] = i as i32;
                last = v;
                last_pos = i as i32;
            }
        }
        result
    }
}
