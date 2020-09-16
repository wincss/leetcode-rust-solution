use crate::*;

use std::collections::VecDeque;

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut indegree = vec![0; n + 1];
        let mut has_indegree_2 = false;
        let mut e = vec![vec![]; n + 1];

        for edge in edges.iter() {
            indegree[edge[1] as usize] += 1;
            if indegree[edge[1] as usize] == 2 {
                has_indegree_2 = true;
            }
            e[edge[0] as usize].push(edge[1] as usize);
        }
        let mut start = edges[0][0] as usize;
        if has_indegree_2 {
            for i in 1..n {
                if indegree[i] == 0 {
                    start = i;
                }
            }
        };
        let mut queue = VecDeque::new();
        queue.push_back(start);
        indegree[start] = -1;
        while let Some(v) = queue.pop_front() {
            for &d in e[v].iter() {
                if indegree[d] < 0 {
                    return vec![v as i32, d as i32];
                }
                indegree[d] = indegree[v] - 1;
                queue.push_back(d);
            }
        }
        unreachable!()
    }
}
