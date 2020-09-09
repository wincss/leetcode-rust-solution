use crate::*;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn generate(
            candidates: &[i32],
            target: i32,
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
                generate(&candidates[1..], target, current, result);
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
        generate(&candidates[..], target, &mut current, &mut result);
        result
    }
}
