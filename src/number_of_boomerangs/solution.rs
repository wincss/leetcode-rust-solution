use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut groups = HashMap::new();
        for i in 0..n {
            for j in i + 1..n {
                let distance = (points[i][0] - points[j][0]) * (points[i][0] - points[j][0])
                    + (points[i][1] - points[j][1]) * (points[i][1] - points[j][1]);
                *groups.entry((i, distance)).or_insert(0) += 1;
                *groups.entry((j, distance)).or_insert(0) += 1;
            }
        }
        // println!("{:?}", groups);
        let mut s = 0;
        for (_, &num) in groups.iter() {
            s += num * (num - 1);
        }
        s
    }
}
