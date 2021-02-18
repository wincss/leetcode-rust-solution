use crate::*;

impl Solution {
    pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut left = 0;
        let mut change = 0;
        let mut result = 0;
        for right in 1..=n {
            if a[right - 1] == 0 {
                change += 1;
            }
            while change > k {
                if a[left] == 0 {
                    change -= 1;
                }
                left += 1;
            }
            result = result.max(right - left)
        }
        result as i32
    }
}
