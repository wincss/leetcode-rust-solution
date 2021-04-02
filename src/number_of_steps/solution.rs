use crate::*;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut step = 0;
        while num > 0 {
            if num & 1 == 1 {
                num -= 1;
            } else {
                num >>= 1;
            }

            step += 1;
        }
        step
    }
}
