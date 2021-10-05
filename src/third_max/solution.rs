use crate::*;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut b1 = None;
        let mut b2 = None;
        let mut b3 = None;
        for i in nums.into_iter() {
            if b1.is_none() || i > b1.unwrap() {
                b3 = b2.take();
                b2 = b1.take();
                b1 = Some(i);
            } else if i < b1.unwrap() && (b2.is_none() || i > b2.unwrap()) {
                b3 = b2.take();
                b2 = Some(i);
            } else if i < b1.unwrap() && i < b2.unwrap() && (b3.is_none() || i > b3.unwrap()) {
                b3 = Some(i);
            }
        }
        b3.unwrap_or(b1.unwrap())
    }
}
