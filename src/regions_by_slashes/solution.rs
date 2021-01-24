use crate::*;

use common::algorithms::union_find::*;
use std::collections::HashMap;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        // \ 1 /
        // 2 X 3
        // / 4 \
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut regions = 0;

        for (i, row) in grid.iter().enumerate() {
            for (j, c) in row.char_indices() {
                match c {
                    ' ' => {
                        union(&(i, j, 1), &(i, j, 2), &mut parent, &mut size);
                        union(&(i, j, 1), &(i, j, 3), &mut parent, &mut size);
                        union(&(i, j, 1), &(i, j, 4), &mut parent, &mut size);
                        regions += 1;
                    }
                    '/' => {
                        union(&(i, j, 1), &(i, j, 2), &mut parent, &mut size);
                        union(&(i, j, 3), &(i, j, 4), &mut parent, &mut size);
                        regions += 2;
                    }
                    '\\' => {
                        union(&(i, j, 1), &(i, j, 3), &mut parent, &mut size);
                        union(&(i, j, 2), &(i, j, 4), &mut parent, &mut size);
                        regions += 2;
                    }
                    _ => unreachable!(),
                }
                if i > 0 {
                    if union(&(i, j, 1), &(i - 1, j, 4), &mut parent, &mut size) {
                        regions -= 1;
                    }
                }
                if j > 0 {
                    if union(&(i, j, 2), &(i, j - 1, 3), &mut parent, &mut size) {
                        regions -= 1;
                    }
                }
            }
        }
        regions
    }
}
