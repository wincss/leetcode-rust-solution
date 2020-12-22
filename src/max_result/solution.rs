use crate::*;

use std::collections::VecDeque;
impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut result = nums[0];

        let mut q = VecDeque::new();
        q.push_back((0, result));
        for i in 1..n {
            while let Some(&(idx, _)) = q.front() {
                if idx + k < i {
                    q.pop_front();
                } else {
                    break;
                }
            }
            let &(_, val) = q.front().unwrap();
            result = val + nums[i];
            while let Some(&(_, last)) = q.back() {
                if result > last {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back((i, result));
            // println!("i={}, q={:?}", i, q);
        }
        result
    }
}
