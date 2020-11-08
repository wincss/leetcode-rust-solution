use crate::*;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        points.sort_by_key(|v| v[0] * v[0] + v[1] * v[1]);
        points.into_iter().take(k as usize).collect()
    }
}
