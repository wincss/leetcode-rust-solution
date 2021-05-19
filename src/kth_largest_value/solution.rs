use crate::*;

use std::collections::BinaryHeap;
impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut matrix = matrix;
        let n = matrix.len();
        let m = matrix[0].len();
        for j in 0..m {
            let mut last = 0;
            for i in 0..n {
                last ^= matrix[i][j];
                matrix[i][j] = last;
            }
        }
        let mut maxk = BinaryHeap::new();
        for i in 0..n {
            let mut last = 0;
            for j in 0..m {
                last ^= matrix[i][j];
                maxk.push(-last);
                if maxk.len() > k {
                    maxk.pop();
                }
            }
        }
        -maxk.pop().unwrap()
    }
}
