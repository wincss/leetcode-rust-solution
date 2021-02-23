use crate::*;

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut a = a;
        let n = a.len();
        let m = a[0].len();
        for i in 0..n {
            for j in 0..(m + 1) / 2 {
                let tmp = a[i][j];
                a[i][j] = 1 - a[i][m - 1 - j];
                if j == m - 1 - j {
                    continue;
                }
                a[i][m - 1 - j] = 1 - tmp;
            }
        }
        a
    }
}
