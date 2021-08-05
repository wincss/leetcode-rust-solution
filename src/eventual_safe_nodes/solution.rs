use crate::*;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        #[derive(Clone, PartialEq)]
        enum Tag {
            White,
            Black,
            Gray,
        }
        let n = graph.len();
        fn check(i: usize, flag: &mut Vec<Tag>, graph: &Vec<Vec<i32>>) -> bool {
            if flag[i] != Tag::White {
                return flag[i] == Tag::Black;
            }
            flag[i] = Tag::Gray;
            for &next in graph[i].iter() {
                if !check(next as usize, flag, graph) {
                    return false;
                }
            }
            flag[i] = Tag::Black;
            return true;
        }
        let mut flag = vec![Tag::White; n];
        (0..n as i32)
            .filter(|&v| check(v as usize, &mut flag, &graph))
            .collect()
    }
}
