use crate::*;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut last = 0;
        let mut vote = 0;
        for &i in nums.iter() {
            if vote == 0 {
                last = i;
            }
            if i == last {
                vote += 1;
            } else {
                vote -= 1;
            }
        }
        // println!("{:?} {}", nums, last);
        vote = 0;
        for &i in nums.iter() {
            if i == last {
                vote += 1
            }
        }
        if vote > nums.len() / 2 {
            last
        } else {
            -1
        }
    }
}
