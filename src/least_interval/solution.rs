use crate::*;

use std::collections::{BinaryHeap, HashMap, VecDeque};
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut h = HashMap::new();
        for c in tasks.into_iter() {
            *h.entry(c).or_insert(0) += 1;
        }
        let mut available = BinaryHeap::new();
        for (c, v) in h.into_iter() {
            available.push((v, c));
        }
        let mut t = 0;
        let mut deferred = VecDeque::new();
        while !(available.is_empty() && deferred.is_empty()) {
            if let Some(&(t0, c, v)) = deferred.front() {
                if t0 <= t {
                    deferred.pop_front();
                    available.push((v, c));
                }
            }
            if let Some((v, c)) = available.pop() {
                t += 1;
                if v > 1 {
                    deferred.push_back((t + n, c, v - 1));
                }
            } else {
                let (t0, c, v) = deferred.pop_front().unwrap();
                t = t0;
                available.push((v, c));
            }
        }
        t
    }
}
