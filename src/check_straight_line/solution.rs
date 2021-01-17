use crate::*;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let mut coordinates = coordinates;
        let p1 = coordinates.pop().unwrap();
        let p2 = coordinates.pop().unwrap();
        if p1[0] == p2[0] {
            for p in coordinates {
                if p[0] != p1[0] {
                    return false;
                }
            }
        } else {
            let k = p2[1] - p1[1];
            let b = p2[0] * p1[1] - p1[0] * p2[1];
            for p in coordinates {
                if k * p[0] + b != (p2[0] - p1[0]) * p[1] {
                    return false;
                }
            }
        }
        true
    }
}
