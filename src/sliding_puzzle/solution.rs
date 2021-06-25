use crate::*;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let finish: Vec<i32> = vec![1, 2, 3, 4, 5, 0];
        let neighbors: Vec<Vec<usize>> = vec![
            vec![1, 3],
            vec![0, 2, 4],
            vec![1, 5],
            vec![0, 4],
            vec![3, 5, 1],
            vec![2, 4],
        ];
        let mut b = vec![];
        for item in board.into_iter() {
            b.extend(item.into_iter());
        }

        let mut q = VecDeque::new();
        let mut v = HashSet::new();
        q.push_back((0, b.clone()));
        v.insert(b);
        while let Some((step, current)) = q.pop_front() {
            if current == finish {
                return step;
            }
            for (idx, &val) in current.iter().enumerate() {
                if val == 0 {
                    let mut current = current.clone();
                    for &idx2 in neighbors[idx].iter() {
                        current[idx] = current[idx2];
                        current[idx2] = 0;
                        if !v.contains(&current) {
                            q.push_back((step + 1, current.clone()));
                            v.insert(current.clone());
                        }
                        current[idx2] = current[idx];
                        current[idx] = 0;
                    }
                }
            }
        }
        -1
    }
}
