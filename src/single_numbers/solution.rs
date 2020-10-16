use crate::*;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut s = 0;
        for &i in nums.iter() {
            s ^= i;
        }
        let diff = s & -s;
        let mut a = 0;
        for &i in nums.iter() {
            if diff & i > 0 {
                a ^= i;
            }
        }
        vec![a, s ^ a]
    }
}
