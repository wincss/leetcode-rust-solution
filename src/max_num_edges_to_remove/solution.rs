use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut ans = 0;
        for edge in edges.iter() {
            if edge[0] == 3 {
                if !union(&edge[1], &edge[2], &mut parent, &mut size) {
                    ans += 1;
                }
            }
        }
        let mut saved_parent = parent.clone();
        let mut saved_size = size.clone();
        for edge in edges.iter() {
            if edge[0] == 1 {
                if !union(&edge[1], &edge[2], &mut parent, &mut size) {
                    ans += 1;
                }
            }
        }
        let ancestor = find(&1, &mut parent, &mut size);
        if size[&ancestor] != n {
            return -1;
        }
        for edge in edges.iter() {
            if edge[0] == 2 {
                if !union(&edge[1], &edge[2], &mut saved_parent, &mut saved_size) {
                    ans += 1;
                }
            }
        }
        let ancestor = find(&1, &mut saved_parent, &mut saved_size);
        if saved_size[&ancestor] != n {
            return -1;
        }
        ans
    }
}
