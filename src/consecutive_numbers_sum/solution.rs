use crate::*;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        // (2 * m + k) * (k + 1) = 2 * n
        // k >= 0
        // m >= 1
        let mut result = 0;
        let n = 2 * n;
        for i in 1..(n as f32).sqrt().ceil() as i32 {
            if n % i == 0 {
                let v = n / i - i + 1;
                if v >= 2 && v & 1 == 0 {
                    result += 1;
                }
            }
        }
        result
    }
}
