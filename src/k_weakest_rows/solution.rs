use crate::*;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = mat.len();
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_by_key(|&v| mat[v].iter().fold(0, |acc, &x| acc + x));
        idx[..k].iter().map(|v| *v as i32).collect()
    }
}
