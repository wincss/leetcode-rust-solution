use crate::*;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        if n == 0 {
            return;
        }
        let m = matrix[0].len();
        let mut r0 = false;
        for i in 0..m {
            if matrix[0][i] == 0 {
                r0 = true;
                break;
            }
        }
        let mut c0 = false;
        for i in 0..n {
            if matrix[i][0] == 0 {
                c0 = true;
                break;
            }
        }
        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 1..n {
            for j in 1..m {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        for i in 0..m {
            if r0 {
                matrix[0][i] = 0;
            }
        }
        for i in 0..n {
            if c0 {
                matrix[i][0] = 0;
            }
        }
    }
}
