use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

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

        let mut parent = HashMap::new();
        let mut size = HashMap::new();

        let mut edge_idx = 0_usize;
        for (i, v) in queries {
            while edge_idx < n && edges[edge_idx][2] < v[2] {
                union(
                    &edges[edge_idx][0],
                    &edges[edge_idx][1],
                    &mut parent,
                    &mut size,
                );
                edge_idx += 1;
            }
            result[i] = find(&v[0], &mut parent, &mut size) == find(&v[1], &mut parent, &mut size);
        }

        result
    }
}
