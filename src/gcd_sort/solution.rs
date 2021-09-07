use crate::*;

use crate::common::algorithms::union_find::*;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn gcd_sort(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let m = *nums.iter().max().unwrap();
        let mut s = HashSet::new();
        for i in 0..n {
            s.insert(nums[i]);
        }
        let mut prime = HashSet::new();
        for i in 2..m {
            prime.insert(i);
        }
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for v in 2..m {
            if prime.contains(&v) {
                for i in 2..(m / v) + 1 {
                    prime.remove(&(i * v));
                    if s.contains(&(i * v)) {
                        union(&(i * v), &v, &mut parent, &mut size);
                    }
                }
            }
        }
        let mut nums2 = nums.clone();
        nums2.sort();
        for i in 0..n {
            if nums[i] != nums2[i]
                && find(&nums[i], &mut parent, &mut size) != find(&nums2[i], &mut parent, &mut size)
            {
                return false;
            }
        }
        true
    }
}
