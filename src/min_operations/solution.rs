use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn min_operations_1713(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let n = target.len();
        let mut position = HashMap::new();
        for (idx, v) in target.into_iter().enumerate() {
            position.insert(v, idx);
        }
        let mut d = vec![];
        for v in arr.into_iter() {
            if let Some(&idx) = position.get(&v) {
                let inpos = d.binary_search(&idx).unwrap_or_else(|x| x);
                if inpos < d.len() {
                    d[inpos] = idx;
                } else {
                    d.push(idx);
                }
            }
        }
        (n - d.len()) as i32
    }

    pub fn min_operations_1827(nums: Vec<i32>) -> i32 {
        let mut op = 0;
        let mut last = 0;
        for i in nums {
            if last >= i {
                last = last + 1;
                op += last - i;
            } else {
                last = i;
            }
        }
        op
    }
}
