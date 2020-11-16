use crate::*;

impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut points = vec![];
        for i in 0..r {
            for j in 0..c {
                points.push(vec![i, j]);
            }
        }
        points.sort_unstable_by_key(|v| (v[0] - r0).abs() + (v[1] - c0).abs());
        points
    }
}
