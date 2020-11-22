use crate::*;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort();
        let mut shots = 0;
        while let Some(v) = points.pop() {
            shots += 1;
            let mut left = v[0];
            let mut right = v[1];
            while let Some(n) = points.last() {
                if left > n[1] || right < n[0] {
                    break;
                }
                left = std::cmp::max(left, n[0]);
                right = std::cmp::min(right, n[1]);
                points.pop();
            }
        }
        shots
    }
}
