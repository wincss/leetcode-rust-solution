use crate::*;

impl Solution {
    // rob in origin problem
    pub fn rob_ii(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut rob_fl = 0;
        let mut rob_fn = 0;
        let mut rob_l = 0;
        let mut rob_n = 0;
        for i in 0..n {
            if i == 0 {
                rob_fl = nums[0];
            } else if i == n - 1 {
                let tmp = rob_n + nums[i];
                rob_n = rob_n.max(rob_l);
                rob_l = tmp;
            } else {
                let tmp = rob_fn + nums[i];
                rob_fn = rob_fn.max(rob_fl);
                rob_fl = tmp;

                let tmp = rob_n + nums[i];
                rob_n = rob_n.max(rob_l);
                rob_l = tmp;
            }
        }
        rob_fl.max(rob_fn).max(rob_l).max(rob_n)
    }
}
