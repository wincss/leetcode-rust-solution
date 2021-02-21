use crate::*;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut sx = n;
        let mut sy = 0;
        for _ in 0..m + n - 1 {
            if sx > 0 {
                sx -= 1;
            } else {
                sy += 1;
            }
            let mut cx = sx;
            let mut cy = sy;
            let v = matrix[cx][cy];
            while cx < n && cy < m {
                if matrix[cx][cy] != v {
                    return false;
                }
                cx += 1;
                cy += 1;
            }
        }
        true
    }
}
