use crate::*;

use std::collections::BinaryHeap;
impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut cp: Vec<_> = capital.into_iter().zip(profits.into_iter()).collect();
        cp.sort();
        let mut w = w;
        let mut cpx = 0;
        let mut h = BinaryHeap::new();
        for _ in 0..k {
            while cpx < cp.len() && cp[cpx].0 <= w {
                h.push(cp[cpx].1);
                cpx += 1;
            }
            if let Some(v) = h.pop() {
                w += v;
            } else {
                break;
            }
        }
        w
    }
}
