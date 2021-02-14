use crate::*;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        fn check(k: i32, nums: &Vec<i32>, maxop: i32) -> bool {
            let mut op = 0;
            for &i in nums.iter() {
                op += (i - 1) / k;
            }
            op <= maxop
        }
        let mut l = 1;
        let mut r = nums.iter().max().unwrap().clone();
        while l < r {
            let m = (l + r) / 2;
            if check(m, &nums, max_operations) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}
