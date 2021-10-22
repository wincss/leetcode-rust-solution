use crate::*;

impl Solution {
    pub fn majority_element_one_third(nums: Vec<i32>) -> Vec<i32> {
        let mut last1 = 0;
        let mut last2 = 0;
        let mut vote1 = 0;
        let mut vote2 = 0;
        for &i in nums.iter() {
            if i == last1 {
                vote1 += 1;
            } else if i == last2 {
                vote2 += 1;
            } else if vote1 == 0 {
                vote1 = 1;
                last1 = i;
            } else if vote2 == 0 {
                vote2 = 1;
                last2 = i;
            } else {
                vote1 -= 1;
                vote2 -= 1;
            }
        }
        let count1 = nums.iter().filter(|&&v| v == last1).count();
        let count2 = nums.iter().filter(|&&v| v == last2).count();
        let mut result = vec![];
        if vote1 > 0 && count1 > nums.len() / 3 {
            result.push(last1);
        }
        if vote2 > 0 && count2 > nums.len() / 3 {
            result.push(last2);
        }
        result
    }
    pub fn majority_element_half(nums: Vec<i32>) -> i32 {
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
