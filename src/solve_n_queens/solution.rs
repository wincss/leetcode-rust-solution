use crate::*;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn convert(current: &Vec<usize>, n: usize) -> Vec<String> {
            let mut result = vec![];
            for &i in current.iter() {
                let mut s = String::new();
                s.push_str(&".".repeat(i));
                s.push('Q');
                s.push_str(&".".repeat(n - i - 1));
                result.push(s)
            }
            result
        }
        fn fill(
            k: usize,
            n: usize,
            cols: i32,
            mdia: i32,
            adia: i32,

            current: &mut Vec<usize>,
            result: &mut Vec<Vec<String>>,
        ) {
            if k == n {
                result.push(convert(current, n));
                return;
            }
            for i in 0..n {
                if cols & (1 << i) > 0 {
                    continue;
                }
                if mdia & (1 << (n - 1 + i - k)) > 0 {
                    continue;
                }
                if adia & (1 << (i + k)) > 0 {
                    continue;
                }
                current[k] = i;
                fill(
                    k + 1,
                    n,
                    cols | (1 << i),
                    mdia | (1 << (n - 1 + i - k)),
                    adia | (1 << (i + k)),
                    current,
                    result,
                );
            }
        }
        let mut current = vec![0; n as usize];
        let mut result = vec![];
        fill(0, n as usize, 0, 0, 0, &mut current, &mut result);
        result
    }
}
