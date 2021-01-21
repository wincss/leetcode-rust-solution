use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut inedge = vec![0; n + 1];

        let mut collision = None;
        let mut cycle = None;

        for edge in edges.iter() {
            if inedge[edge[1] as usize] > 0 {
                collision = Some(edge.clone());
            } else {
                inedge[edge[1] as usize] = edge[0];
                if !union(&edge[0], &edge[1], &mut parent, &mut size) {
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
