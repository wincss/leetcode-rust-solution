use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let mut result = 0;
        let mut right_min = 0;
        let mut right_max = 0;
        let mut distinct_min = 0;
        let mut distinct_max = 0;
        let mut h_min = HashMap::new();
        let mut h_max = HashMap::new();

        for i in 0..n {
            if i > 0 {
                h_min.entry(a[i - 1]).and_modify(|v| {
                    *v -= 1;
                    if *v == 0 {
                        distinct_min -= 1;
                    }
                });
                h_max.entry(a[i - 1]).and_modify(|v| {
                    *v -= 1;
                    if *v == 0 {
                        distinct_max -= 1;
                    }
                });
            }
            while distinct_min < k && right_min < n {
                *h_min.entry(a[right_min]).or_insert(0) += 1;
                if h_min[&a[right_min]] == 1 {
                    distinct_min += 1;
                }
                right_min += 1;
            }
            while distinct_max <= k && right_max < n {
                *h_max.entry(a[right_max]).or_insert(0) += 1;
                if h_max[&a[right_max]] == 1 {
                    distinct_max += 1;
                }
                right_max += 1;
            }
            // println!("{} {:?} {} {:?} {}", i, h_min, right_min, h_max, right_max);

            result += right_max - right_min;
            if distinct_max == k {
                result += 1;
            }
        }
        result as i32
    }
}
