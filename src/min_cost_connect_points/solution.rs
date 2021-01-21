use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = vec![];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let distance =
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edges.push((distance, i, j));
            }
        }
        edges.sort();
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut result = 0;
        for (d, x, y) in edges {
            if union(&x, &y, &mut parent, &mut size) {
                result += d;
            }
        }
        result
    }
}
