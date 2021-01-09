use crate::*;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut start = 0;
        let mut end = None;
        let mut result = vec![];
        let n = nums.len();
        for i in 0..=n {
            if i == n || end.is_none() || end.unwrap() + 1 != nums[i] {
                match end {
                    Some(v) if v == start => {
                        result.push(format!("{}", start));
                    }
                    Some(v) => {
                        result.push(format!("{}->{}", start, v));
                    }
                    None => {}
                }
                if i == n {
                    break;
                }
                start = nums[i];
            }
            end = Some(nums[i]);
            // println!("{}, {:?}", start, end);
        }
        result
    }
}
