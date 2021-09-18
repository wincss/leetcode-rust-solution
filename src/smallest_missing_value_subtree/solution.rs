use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        fn bfs(
            node: usize,
            tree: &Vec<Vec<usize>>,
            nums: &Vec<i32>,
            values: &mut Vec<bool>,
            exclude_child: usize,
        ) {
            let mut queue = VecDeque::new();
            queue.push_back(node);
            while let Some(current) = queue.pop_front() {
                values[nums[current] as usize] = true;
                for &child in tree[current].iter() {
                    if child != exclude_child {
                        queue.push_back(child);
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
                let mut missing = 2;
                let mut parent = idx;
                let mut child = idx;
                loop {
                    bfs(parent, &tree, &nums, &mut v, child);
                    while v[missing] {
                        missing += 1;
                    }

                    result[parent] = missing as i32;
                    if parents[parent] == -1 {
                        break;
                    }
                    child = parent;
                    parent = parents[parent] as usize;
                }
            }
        }
        result
    }
}
