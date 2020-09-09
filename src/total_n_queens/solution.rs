use crate::*;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn fill(k: usize, n: usize, cols: i32, mdia: i32, adia: i32, result: &mut i32) {
            if k == n {
                *result += 1;
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
                fill(
                    k + 1,
                    n,
                    cols | (1 << i),
                    mdia | (1 << (n - 1 + i - k)),
                    adia | (1 << (i + k)),
                    result,
                );
            }
        }
        let mut result = 0;
        fill(0, n as usize, 0, 0, 0, &mut result);
        result
    }
}
