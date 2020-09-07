use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
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
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut ans = 0;
        for edge in edges.iter() {
            if edge[0] == 3 {
                if !union(edge[1], edge[2], &mut parent, &mut size) {
                    ans += 1;
                }
            }
        }
        let mut saved_parent = parent.clone();
        let mut saved_size = size.clone();
        for edge in edges.iter() {
            if edge[0] == 1 {
                if !union(edge[1], edge[2], &mut parent, &mut size) {
                    ans += 1;
                }
            }
        }
        let ancestor = find(1, &mut parent, &mut size);
        if size[&ancestor] != n {
            return -1;
        }
        for edge in edges.iter() {
            if edge[0] == 2 {
                if !union(edge[1], edge[2], &mut saved_parent, &mut saved_size) {
                    ans += 1;
                }
            }
        }
        let ancestor = find(1, &mut saved_parent, &mut saved_size);
        if saved_size[&ancestor] != n {
            return -1;
        }
        ans
    }
}
