use crate::*;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let n = s.len();
        let diff: Vec<i32> = s
            .chars()
            .zip(t.chars())
            .map(|(i, j)| (i as u8 as i32 - j as u8 as i32).abs())
            .collect();
        let mut cost = 0;
        let mut left = 0;
        let mut result = 0;
        for i in 0..n {
            cost += diff[i];
            while cost > max_cost {
                cost -= diff[left];
                left += 1;
            }
            result = std::cmp::max(result, i + 1 - left);
        }
        result as i32
    }
}
