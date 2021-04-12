use crate::*;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<String> = nums.into_iter().map(|v| v.to_string()).collect();
        nums.sort_by_cached_key(|v| v.repeat(10));
        let result: String = nums
            .into_iter()
            .rev()
            .map(|v| v.chars().collect::<Vec<char>>())
            .flatten()
            .skip_while(|c| *c == '0')
            .collect();
        if result.is_empty() {
            "0".to_string()
        } else {
            result
        }
    }
}
