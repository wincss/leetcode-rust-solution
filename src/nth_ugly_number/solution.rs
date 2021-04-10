use crate::*;

use std::collections::BinaryHeap;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut v = BinaryHeap::new();
        v.push(-1_i64);

        let mut last = 0;
        for _ in 0..n {
            while let Some(c) = v.pop() {
                if c != last {
                    last = c;
                    break;
                }
            }
            v.push(last * 2);
            v.push(last * 3);
            v.push(last * 5);
        }
        -last as i32
    }
}
