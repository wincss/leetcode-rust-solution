use crate::*;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }
        let mut m = m;
        let mut n = n;
        if m < n {
            let t = n;
            n = m;
            m = t;
        }
        let mut result = 1_i64;
        for i in 0..n - 1 {
            result *= (m + i) as i64;
            result /= (i + 1) as i64;
        }
        result as i32
    }
}
