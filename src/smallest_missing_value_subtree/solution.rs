use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        fn bfs(
            node: usize,
            tree: &Vec<Vec<usize>>,
            nums: &Vec<i32>,
            values: &mut Vec<bool>,
            visited: &mut Vec<bool>,
        ) {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            visited[node] = true;
            while let Some(current) = queue.pop_front() {
                values[nums[current] as usize] = true;
                for &child in tree[current].iter() {
                    if !visited[child] {
                        queue.push_back(child);
                        visited[child] = true;
                    }
                }
            }
        }
        let n = parents.len();
        let m = nums.iter().max().unwrap().clone();
        let mut tree = vec![vec![]; n];
        for (child, &parent) in parents.iter().enumerate() {
            if parent == -1 {
                continue;
            }
            tree[parent as usize].push(child);
        }
        let mut result = vec![1; n];
        for idx in 0..n {
            if nums[idx] == 1 {
                let mut v = vec![false; 2 + m as usize];
                let mut vv = vec![false; n];
                let mut missing = 2;
                let mut parent = idx;
                loop {
                    bfs(parent, &tree, &nums, &mut v, &mut vv);
                    while v[missing] {
                        missing += 1;
                    }

                    result[parent] = missing as i32;
                    if parents[parent] == -1 {
                        break;
                    }
                    parent = parents[parent] as usize;
                }
            }
        }
        result
    }
}
