use crate::*;

use common::algorithms::union_find::UnionFind;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = UnionFind::new();
        for mut edge in edges {
            if !graph.union(&edge[0], &edge[1]) {
                edge.sort();
                return edge;
            }
        }
        unreachable!();
    }
}
