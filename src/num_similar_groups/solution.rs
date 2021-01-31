use crate::*;

use common::algorithms::union_find::*;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let h: HashSet<String> = strs.into_iter().collect();
        let strs: Vec<String> = h.into_iter().collect();

        fn check_two_swap(s1: &String, s2: &String) -> bool {
            let mut diff = 0;
            for (x, y) in s1.chars().zip(s2.chars()) {
                if x != y {
                    diff += 1;
                }
            }
            diff <= 2
        }
        let n = strs.len();
        let mut result = n as i32;
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for i in 0..n {
            for j in i + 1..n {
                if check_two_swap(&strs[i], &strs[j]) {
                    if union(&&strs[i], &&strs[j], &mut parent, &mut size) {
                        result -= 1;
                    }
                }
            }
        }
        result
    }
}
