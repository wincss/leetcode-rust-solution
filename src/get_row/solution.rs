use crate::*;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut result = vec![1; n + 1];
        for i in 1..=n {
            for j in (1..i).rev() {
                result[j] += result[j - 1];
            }
        }
        result
    }
}
