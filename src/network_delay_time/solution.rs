use crate::*;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut s = 0;
        let mut maxtm = 0;
        let mut v = vec![false; n + 1];
        let mut m = HashMap::new();
        for item in times.into_iter() {
            m.entry(item[0] as usize)
                .or_insert(HashMap::new())
                .insert(item[1] as usize, item[2]);
        }
        let mut t = BinaryHeap::new();
        t.push((Reverse(0), k));
        while let Some((Reverse(tm), c)) = t.pop() {
            if v[c] {
                continue;
            }
            maxtm = maxtm.max(tm);
            v[c] = true;
            s += 1;
            if !m.contains_key(&c) {
                continue;
            }
            for (&next, &cost) in m[&c].iter() {
                t.push((Reverse(tm + cost), next));
            }
        }
        if s < n {
            -1
        } else {
            maxtm
        }
    }
}
