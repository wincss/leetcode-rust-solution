use crate::*;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut d: Vec<i32> = vec![];
        let mut result = vec![0; n];
        for i in (0..n).rev() {
            let mut left = 0;
            let mut right = d.len();
            while left < right {
                let mid = (left + right) / 2;
                if d[mid] >= heights[i] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if left > 0 {
                left -= 1;
            }
            result[i] = (d.len() - left) as i32;
            while !d.is_empty() && *d.last().unwrap() <= heights[i] {
                d.pop();
            }
            d.push(heights[i]);
        }
        result
    }
}
