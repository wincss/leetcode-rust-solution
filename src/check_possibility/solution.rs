use crate::*;

impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut changed = false;
        let mut last = std::i32::MIN;
        let mut last2 = std::i32::MIN;
        for i in nums {
            if i < last {
                if changed {
                    return false;
                }
                changed = true;
                if i >= last2 {
                    last = i
                }
            } else {
                last2 = last;
                last = i;
            }
        }
        true
    }
}
