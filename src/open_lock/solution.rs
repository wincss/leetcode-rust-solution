use crate::*;

use std::collections::{HashSet, VecDeque};
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut d = HashSet::new();
        for s in deadends.into_iter() {
            let t: i32 = s.parse().unwrap();
            if t == 0 {
                return -1;
            }
            d.insert(t);
        }
        let mut v = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        v.insert(0);
        while let Some((step, current)) = q.pop_front() {
            // println!("{}, {}", step, current);
            if format!("{:04}", current) == target {
                return step;
            }
            for &i in &[1, 10, 100, 1000] {
                let a = current / 10 / i * 10 * i + current % i;
                let b = current - a;
                let n1 = a + (b + i) % (i * 10);
                if !d.contains(&n1) && !v.contains(&n1) {
                    q.push_back((step + 1, n1));
                    v.insert(n1);
                }
                let n2 = a + (b + 9 * i) % (i * 10);
                if !d.contains(&n2) && !v.contains(&n2) {
                    q.push_back((step + 1, n2));
                    v.insert(n2);
                }
            }
        }
        -1
    }
}
