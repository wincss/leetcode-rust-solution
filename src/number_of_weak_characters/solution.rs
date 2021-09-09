use crate::*;

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        let mut p = properties;
        p.sort();
        let n = p.len();
        let mut max_eq = p[n - 1][1];
        let mut max_gt = 0;
        let mut count = 0;
        for i in (0..n - 1).rev() {
            if p[i][0] < p[i + 1][0] {
                max_gt = max_gt.max(max_eq);
                max_eq = 0;
            }
            if max_gt > p[i][1] {
                count += 1;
            }
            max_eq = max_eq.max(p[i][1]);
        }
        count
    }
}
