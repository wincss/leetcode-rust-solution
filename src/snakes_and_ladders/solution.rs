use crate::*;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let mut q = VecDeque::new();
        let mut v = HashSet::new();
        q.push_back((0, 1));
        v.insert(1);
        while let Some((step, current)) = q.pop_front() {
            if current == n * n {
                return step;
            }
            for i in 1..=6 {
                let mut target = current + i;
                if target > n * n {
                    break;
                }
                let r = (target - 1) / n;
                let c = (target - 1) % n;
                let c = if r & 1 == 0 { c } else { n - 1 - c };
                let r = n - 1 - r;
                if board[r][c] != -1 {
                    target = board[r][c] as usize;
                }
                if !v.contains(&target) {
                    q.push_back((step + 1, target));
                    v.insert(target);
                }
            }
        }
        -1
    }
}
