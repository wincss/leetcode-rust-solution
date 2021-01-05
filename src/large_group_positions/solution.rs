use crate::*;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let n = s.len();
        let mut result = vec![];
        let mut last_char = None;
        let mut last_start = None;
        for (i, c) in s.chars().enumerate() {
            if !last_char.map(|v| v == c).unwrap_or(false) {
                if last_start.map(|v| i - v >= 3).unwrap_or(false) {
                    result.push(vec![last_start.unwrap() as i32, i as i32 - 1])
                }
                last_char = Some(c);
                last_start = Some(i);
            }
        }
        if n - last_start.unwrap_or(n) >= 3 {
            result.push(vec![last_start.unwrap() as i32, n as i32 - 1]);
        }
        result
    }
}
