use crate::*;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        assert!(n > 0);
        let m = matrix[0].len();
        let count = m * n;
        let mut left = 0;
        let mut right = count - 1;
        while left < right {
            let mid = (left + right) / 2;
            let x = mid / m;
            let y = mid % m;
            if matrix[x][y] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        matrix[left / m][left % m] == target
    }
}
