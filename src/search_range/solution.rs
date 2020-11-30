use crate::*;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let m = (l + r) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if l >= nums.len() || nums[l] != target {
            return vec![-1, -1];
        }
        let left = l;
        l = 0;
        r = nums.len() - 1;
        while l < r {
            let m = (l + r + 1) / 2;
            if nums[m] > target {
                r = m - 1;
            } else {
                l = m;
            }
        }
        vec![left as i32, l as i32]
    }
}
