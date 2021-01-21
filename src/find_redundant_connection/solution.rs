use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for mut edge in edges {
            if !union(&edge[0], &edge[1], &mut parent, &mut size) {
                edge.sort();
                return edge;
            }
        }
        unreachable!();
    }
}
