use crate::*;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let l = nums.len();
        let mut cover: i64 = 1;
        let mut to_add = 0;
        let mut idx = 0;

        while cover <= n as i64 {
            if idx < l && nums[idx] as i64 <= cover {
                cover += nums[idx] as i64;
                idx += 1;
            } else {
                cover *= 2;
                to_add += 1;
            }
        }
        to_add
    }
}
