use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut col = HashMap::new();
        let mut row = HashMap::new();
        let mut removed = 0;
        for (i, stone) in stones.iter().enumerate() {
            let old = col.entry(&stone[0]).or_insert(i);
            if union(old, &i, &mut parent, &mut size) {
                removed += 1;
            }
            let old = row.entry(&stone[1]).or_insert(i);
            if union(old, &i, &mut parent, &mut size) {
                removed += 1;
            }
        }
        removed
    }
}
