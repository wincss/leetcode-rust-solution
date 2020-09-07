use crate::*;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn generate(
            n: i32,
            k: i32,
            now: i32,
            start: i32,
            current: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if now == k {
                result.push(current.clone());
                return;
            }
            for i in start..=n {
                current.push(i);
                generate(n, k, now + 1, i + 1, current, result);
                current.pop();
            }
        }
        let mut current = Vec::new();
        let mut result = Vec::new();
        generate(n, k, 0, 1, &mut current, &mut result);
        result
    }
}
