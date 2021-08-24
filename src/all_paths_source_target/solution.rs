use crate::*;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn go(now: i32, graph: &Vec<Vec<i32>>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if now == graph.len() as i32 - 1 {
                result.push(current.clone());
                return;
            }
            for &next in graph[now as usize].iter() {
                current.push(next);
                go(next, graph, current, result);
                current.pop();
            }
        }
        let mut current = vec![0];
        let mut result = vec![];
        go(0, &graph, &mut current, &mut result);
        result
    }
}
