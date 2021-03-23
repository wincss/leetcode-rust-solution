use crate::*;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut left_min = vec![];
        let mut right_max = vec![];
        let mut last = std::i32::MAX;
        for (i, &v) in nums.iter().enumerate() {
            while !right_max.is_empty() && nums[*right_max.last().unwrap()] <= v {
                right_max.pop();
            }
            if let Some(&k) = right_max.last() {
                if v > left_min[k] {
                    return true;
                }
            }
            right_max.push(i);
            last = last.min(v);
            left_min.push(last);
        }
        false
    }
}
