use crate::*;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut op = 0;
        let mut last = 0;
        for i in nums {
            if last >= i {
                last = last + 1;
                op += last - i;
            } else {
                last = i;
            }
        }
        op
    }
}
