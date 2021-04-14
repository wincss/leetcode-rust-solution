use crate::*;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut rob_l = 0;
        let mut rob_n = 0;
        for i in 0..n {
            let tmp = rob_n + nums[i];
            rob_n = rob_n.max(rob_l);
            rob_l = tmp;
        }
        rob_l.max(rob_n)
    }
}
