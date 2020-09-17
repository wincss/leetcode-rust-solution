use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn generate(
            n: usize,
            uniq: &Vec<i32>,
            candidates: &mut HashMap<i32, i32>,
            last: Option<i32>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current.len() == n {
                result.push(current.clone());
                return;
            }
            for &k in uniq {
                if last.is_some() && last.unwrap() == k {
                    continue;
                }
                let v = candidates[&k];
                for _ in 0..v {
                    current.push(k);
                    candidates.entry(k).and_modify(|v| *v -= 1);
                    generate(n, uniq, candidates, Some(k), current, result);
                }
                for _ in 0..v {
                    current.pop();
                }
                candidates.insert(k, v);
            }
        }
        let n = nums.len();
        let mut candidates = HashMap::new();
        for i in nums {
            *candidates.entry(i).or_insert(0) += 1;
        }
        let uniq = candidates.keys().map(|v| *v).collect::<Vec<i32>>();
        let mut current = vec![];
        let mut result = vec![];
        generate(n, &uniq, &mut candidates, None, &mut current, &mut result);
        result
    }
}
