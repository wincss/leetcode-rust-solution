use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let n = nums.len();
        nums.sort();
        let mut two_sum = HashMap::new();
        for i in 0..n {
            for j in i + 1..n {
                two_sum
                    .entry(nums[i] + nums[j])
                    .or_insert(HashMap::new())
                    .entry((nums[i], nums[j]))
                    .or_insert(vec![])
                    .push((i, j));
            }
        }
        let mut result = vec![];
        for (&v, p1) in two_sum.iter() {
            if let Some(p2) = two_sum.get(&(target - v)) {
                for (&(v1, v2), x) in p1.iter() {
                    for (&(v3, v4), y) in p2.iter() {
                        let mut available = false;
                        'outer: for &(_, p1_second) in x.iter() {
                            for &(p2_first, _) in y.iter() {
                                if p1_second < p2_first {
                                    available = true;
                                    break 'outer;
                                }
                            }
                        }
                        if available {
                            result.push(vec![v1, v2, v3, v4]);
                        }
                    }
                }
            }
        }
        result
    }
}
