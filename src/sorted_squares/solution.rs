use crate::*;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in a.into_iter() {
            result.push(i * i);
        }
        result.sort();
        result
    }
}
