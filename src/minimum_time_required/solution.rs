use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        fn dp(
            state: usize,
            k: i32,
            sums: &Vec<i32>,
            saved: &mut HashMap<(usize, i32), Option<i32>>,
        ) -> Option<i32> {
            if state == 0 && k == 0 {
                return Some(0);
            }
            if state == 0 || k == 0 {
                return None;
            }
            if let Some(v) = saved.get(&(state, k)) {
                return v.clone();
            }
            let mut set = state;
            let mut result = None;
            while set > 0 {
                if let Some(v) = dp(state - set, k - 1, sums, saved) {
                    let new = v.max(sums[set]);
                    if result.is_none() || result.unwrap() > new {
                        result = Some(new);
                    }
                }
                set = (set - 1) & state;
            }
            saved.insert((state, k), result);
            result
        }
        let n = jobs.len();
        let mut sums = vec![0];
        for i in 1_usize..(1 << n) {
            sums.push(sums[i & (i - 1)] + jobs[i.trailing_zeros() as usize]);
        }
        let mut saved = HashMap::new();
        dp((1 << n) - 1, k, &sums, &mut saved).unwrap()
    }
}
