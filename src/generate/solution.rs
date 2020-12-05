use crate::*;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for i in 1..=num_rows {
            if i == 1 {
                result.push(vec![1]);
            } else {
                let mut current_row = vec![1];
                let last_row = result.last().unwrap();

                for j in 1..last_row.len() {
                    current_row.push(last_row[j - 1] + last_row[j]);
                }
                current_row.push(1);
                result.push(current_row);
            }
        }
        result
    }
}
