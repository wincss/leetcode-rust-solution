use crate::*;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn generate(
            base: i32,
            remain: i32,
            target: i32,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if target == 0 && remain == 0 {
                result.push(current.clone());
                return;
            }
            if base == 10 || target == 0 || remain == 0 {
                return;
            }
            generate(base + 1, remain, target, current, result);
            current.push(base);
            generate(base + 1, remain - 1, target - base, current, result);
            current.pop();
        }
        let mut current = vec![];
        let mut result = vec![];
        generate(1, k, n, &mut current, &mut result);
        result
    }
}
