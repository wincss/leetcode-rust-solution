use crate::*;

impl Solution {
    pub fn minimum_operations(leaves: String) -> i32 {
        let n = leaves.len();
        let mut y = vec![0];
        let mut last = 0;
        for i in leaves.chars() {
            if i == 'y' {
                last += 1;
            }
            y.push(last);
        }
        let mut t = vec![std::i32::MAX; n + 1];
        for i in (0..n).rev() {
            t[i] = std::cmp::min(t[i + 1], i as i32 - 2 * y[i]);
        }
        (1..n - 1)
            .map(|i| y[n] + 2 * y[i] - i as i32 + t[i + 1])
            .min()
            .unwrap()
    }
}
