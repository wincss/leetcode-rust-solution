use crate::*;

use common::algorithms::union_find::UnionFind;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut graph = UnionFind::new();
        let mut inedge = vec![0; n + 1];

        let mut collision = None;
        let mut cycle = None;

        for edge in edges.iter() {
            if inedge[edge[1] as usize] > 0 {
                collision = Some(edge.clone());
            } else {
                inedge[edge[1] as usize] = edge[0];
                if !graph.union(&edge[0], &edge[1]) {
                    cycle = Some(edge.clone());
                }
            }
        }
        if collision.is_none() {
            cycle.unwrap()
        } else if cycle.is_none() {
            collision.unwrap()
        } else {
            let collision = collision.unwrap();
            vec![inedge[collision[1] as usize], collision[1]]
        }
    }
}
