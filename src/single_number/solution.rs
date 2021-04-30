use crate::*;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut t0 = -1;
        let mut t1 = 0;
        let mut t2 = 0;
        for i in nums {
            let v0 = t0 & i;
            let v1 = t1 & i;
            let v2 = t2 & i;

            t2 = (t2 ^ v2) | v1;
            t1 = (t1 ^ v1) | v0;
            t0 = (t0 ^ v0) | v2;
        }
        t1
    }
}
