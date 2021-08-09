use crate::*;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut d: Vec<i32> = vec![];
        let mut result = vec![0; n];
        for i in (0..n).rev() {
            while !d.is_empty() {
                result[i] += 1;
                match *d.last().unwrap() {
                    v if v <= heights[i] => {
                        d.pop();
                    }
                    _ => {
                        break;
                    }
                }
            }
            d.push(heights[i]);
        }
        result
    }
}
