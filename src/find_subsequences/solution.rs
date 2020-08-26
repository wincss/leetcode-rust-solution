use crate::*;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &Vec<i32>, i: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                if current.len() > 1 {
                    result.push(current.clone());
                }
                return;
            }
            if current.len() == 0 || nums[i] >= current[current.len() - 1] {
                current.push(nums[i]);
                helper(nums, i + 1, current, result);
                current.pop();
            }
            if current.len() == 0 || nums[i] != current[current.len() - 1] {
                helper(nums, i + 1, current, result);
            }
        }
        let mut result = vec![];
        let mut current = vec![];
        helper(&nums, 0, &mut current, &mut result);
        result
    }
}
