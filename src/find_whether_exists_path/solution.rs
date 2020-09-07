use crate::*;

use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn find_whether_exists_path(
        _n: i32,
        graph: Vec<Vec<i32>>,
        start: i32,
        target: i32,
    ) -> bool {
        let mut edges = HashMap::new();
        for edge in graph {
            edges
                .entry(edge[0])
                .or_insert(HashSet::new())
                .insert(edge[1]);
        }
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(node) = queue.pop_front() {
            if node == target {
                return true;
            }
            if let Some(dests) = edges.get(&node) {
                for &dest in dests {
                    if visited.insert(dest) {
                        queue.push_back(dest);
                    }
                }
            }
        }
        false
    }
}
