use crate::*;

use std::collections::HashMap;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn find(x: i32, parent: &mut HashMap<i32, i32>, size: &mut HashMap<i32, i32>) -> i32 {
            let f = parent.entry(x).or_insert(x);
            size.entry(x).or_insert(1);
            if *f == x {
                x
            } else {
                let p = find(parent[&x], parent, size);
                parent.insert(x, p);
                parent[&x]
            }
        }
        fn union(
            x: i32,
            y: i32,
            parent: &mut HashMap<i32, i32>,
            size: &mut HashMap<i32, i32>,
        ) -> bool {
            let x = find(x, parent, size);
            let y = find(y, parent, size);
            if x == y {
                return false;
            }
            if size[&x] > size[&y] {
                parent.insert(y, x);
                size.insert(x, size[&x] + size[&y]);
            } else {
                parent.insert(x, y);
                size.insert(y, size[&x] + size[&y]);
            }
            true
        }
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
                if !union(edge[0], edge[1], &mut parent, &mut size) {
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
