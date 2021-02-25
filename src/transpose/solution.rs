use crate::*;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = matrix.len();
        let m = matrix.first().map_or(0, |v| v.len());
        let mut result = vec![vec![0; n]; m];
        for i in 0..n {
            for j in 0..m {
                result[j][i] = matrix[i][j];
            }
        }
        result
    }
}
