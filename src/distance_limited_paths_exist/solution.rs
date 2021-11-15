use crate::*;

use common::algorithms::union_find::UnionFind;

impl Solution {
    pub fn distance_limited_paths_exist(
        _: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = edge_list.len();
        let mut edges = edge_list;
        let mut queries: Vec<_> = queries.into_iter().enumerate().collect();
        edges.sort_by_key(|v| v[2]);
        queries.sort_by_key(|(_, v)| v[2]);

        let mut result = vec![false; queries.len()];

        let mut graph = UnionFind::new();

        let mut edge_idx = 0_usize;
        for (i, v) in queries {
            while edge_idx < n && edges[edge_idx][2] < v[2] {
                graph.union(&edges[edge_idx][0], &edges[edge_idx][1]);
                edge_idx += 1;
            }
            result[i] = graph.find(&v[0]) == graph.find(&v[1]);
        }

        result
    }
}
