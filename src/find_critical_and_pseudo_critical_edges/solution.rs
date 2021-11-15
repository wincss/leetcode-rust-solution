use crate::*;

use common::algorithms::union_find::UnionFind;

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn mst(
            edge_idx: usize,
            include: bool,
            edges: &Vec<Vec<i32>>,
            edge_id: &Vec<usize>,
            n: i32,
        ) -> i32 {
            let mut graph = UnionFind::new();
            let mut weight = 0;
            let mut n = n;
            if include {
                graph.union(&edges[edge_idx][0], &edges[edge_idx][1]);
                weight += edges[edge_idx][2];
                n -= 1;
            }
            for &idx in edge_id.iter() {
                if idx == edge_idx {
                    continue;
                }
                let edge = &edges[idx];
                if graph.union(&edge[0], &edge[1]) {
                    weight += edge[2];
                    n -= 1;
                }
                if n == 1 {
                    break;
                }
            }
            weight
        }
        let m = edges.len();
        let mut edge_id: Vec<_> = (0_usize..m).collect();
        edge_id.sort_by_key(|&v| edges[v][2]);

        let mut result = vec![vec![], vec![]];
        let reference = mst(edge_id[0], true, &edges, &edge_id, n);
        for &idx in edge_id.iter() {
            let without_edge = mst(idx, false, &edges, &edge_id, n);
            let with_edge = mst(idx, true, &edges, &edge_id, n);
            // println!(
            //     "edges[{}]={:?} ref={}, w={}, w/o={}",
            //     idx, edges[idx], reference, with_edge, without_edge
            // );

            if without_edge != reference {
                result[0].push(idx as i32);
            } else if with_edge == reference {
                result[1].push(idx as i32);
            }
        }
        result
    }
}
