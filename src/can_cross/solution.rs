use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        fn dp(idx: usize, k: i32, saved: &mut HashSet<(usize, i32)>, stones: &Vec<i32>) -> bool {
            if saved.contains(&(idx, k)) {
                return false;
            }
            if idx == stones.len() - 1 {
                return true;
            }
            if k > 1 {
                if let Ok(next) = stones.binary_search(&(stones[idx] + k - 1)) {
                    if dp(next, k - 1, saved, stones) {
                        return true;
                    }
                }
            }

            if let Ok(next) = stones.binary_search(&(stones[idx] + k)) {
                if dp(next, k, saved, stones) {
                    return true;
                }
            }

            if let Ok(next) = stones.binary_search(&(stones[idx] + k + 1)) {
                if dp(next, k + 1, saved, stones) {
                    return true;
                }
            }
            saved.insert((idx, k));
            false
        }
        if stones.len() < 2 {
            return true;
        }
        if stones[1] != 1 {
            return false;
        }
        let mut saved = HashSet::new();
        dp(1, 1, &mut saved, &stones)
    }
}
