use crate::*;

use common::algorithms::union_find::*;
use std::collections::HashMap;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len();
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for i in 0..n / 2 {
            union(
                &(row[2 * i] / 2),
                &(row[2 * i + 1] / 2),
                &mut parent,
                &mut size,
            );
        }
        let mut result = 0;
        for i in 0..n as i32 / 2 {
            if find(&i, &mut parent, &mut size) != i {
                result += 1;
            }
        }
        result
    }
}
