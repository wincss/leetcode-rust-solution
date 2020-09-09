use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn generate(
            candidates: &[i32],
            target: i32,
            max_number: &HashMap<i32, i32>,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 {
                result.push(current.clone());
                return;
            }
            if candidates.len() == 0 {
                return;
            }
            let base = candidates[0];
            let mut n = 0;
            let mut target = target;
            while target >= 0 {
                generate(&candidates[1..], target, max_number, current, result);
                if n == max_number[&base] {
                    break;
                }
                n += 1;
                current.push(base);
                target -= base;
            }
            for _ in 0..n {
                current.pop();
            }
        }
        let mut current = vec![];
        let mut result = vec![];
        let mut max_number = HashMap::new();
        for i in candidates {
            let v = max_number.entry(i).or_insert(0);
            *v += 1;
        }
        generate(
            &max_number.keys().map(|v| *v).collect::<Vec<i32>>()[..],
            target,
            &max_number,
            &mut current,
            &mut result,
        );
        result
    }
}
