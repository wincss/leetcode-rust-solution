use crate::*;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut step = 0;
        while num > 1 {
            step += num & 1;
            num >>= 1;
            step += 1;
        }
        num * (step + 1)
    }
}
