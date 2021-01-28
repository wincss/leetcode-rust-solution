use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let n = heights.len();
        if n == 0 {
            return 0;
        }
        let m = heights[0].len();
        if m == 0 || (n == 1 && m == 1) {
            return 0;
        }
        let mut edges = vec![];
        for i in 0..n {
            for j in 0..m {
                if i > 0 {
                    edges.push((
                        (heights[i][j] - heights[i - 1][j]).abs(),
                        (i - 1) * m + j,
                        i * m + j,
                    ));
                }
                if j > 0 {
                    edges.push((
                        (heights[i][j] - heights[i][j - 1]).abs(),
                        i * m + j - 1,
                        i * m + j,
                    ));
                }
            }
        }
        edges.sort();
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let dest = m * n - 1;
        for &(c, x, y) in edges.iter() {
            union(&x, &y, &mut parent, &mut size);
            if find(&0, &mut parent, &mut size) == find(&dest, &mut parent, &mut size) {
                return c;
            }
        }
        unreachable!();
    }
}
