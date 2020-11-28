use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut sab = HashMap::new();
        let mut scd = HashMap::new();
        let n = a.len();
        for i in 0..n {
            for j in 0..n {
                *sab.entry(a[i] + b[j]).or_insert(0) += 1;
            }
        }
        for i in 0..n {
            for j in 0..n {
                *scd.entry(c[i] + d[j]).or_insert(0) += 1;
            }
        }
        let mut ret = 0;
        for &i in sab.keys() {
            if scd.contains_key(&-i) {
                ret += sab[&i] * scd[&-i];
            }
        }
        ret
    }
}
