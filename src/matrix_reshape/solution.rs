use crate::*;

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let n = nums.len();
        let m = nums[0].len();
        let c = c as usize;
        let r = r as usize;
        if r * c != n * m {
            return nums;
        }
        let mut result = vec![vec![]];
        for i in nums.into_iter().flatten() {
            if result.last().unwrap().len() == c {
                result.push(vec![]);
            }
            result.last_mut().unwrap().push(i);
        }
        result
    }
}
