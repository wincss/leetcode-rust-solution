use crate::*;

use common::algorithms::union_find::*;
use std::collections::HashMap;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut nodes = vec![];
        for i in 0..n {
            for j in 0..n {
                nodes.push((grid[i][j], i, j));
            }
        }
        nodes.sort();
        let dest = n * n - 1;
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for (h, x, y) in nodes {
            let key = x * n + y;
            if x > 0 && grid[x - 1][y] <= grid[x][y] {
                union(&key, &(key - n), &mut parent, &mut size);
            }
            if x < n - 1 && grid[x + 1][y] <= grid[x][y] {
                union(&key, &(key + n), &mut parent, &mut size);
            }
            if y > 0 && grid[x][y - 1] <= grid[x][y] {
                union(&key, &(key - 1), &mut parent, &mut size);
            }
            if y < n - 1 && grid[x][y + 1] <= grid[x][y] {
                union(&key, &(key + 1), &mut parent, &mut size);
            }
            if find(&0, &mut parent, &mut size) == find(&dest, &mut parent, &mut size) {
                return h;
            }
        }
        unreachable!();
    }
}
