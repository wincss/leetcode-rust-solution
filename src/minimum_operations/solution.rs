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
        let mut ans = std::i32::MAX;
        for p1 in (1..n - 1).rev() {
            ans = std::cmp::min(ans, y[n] + 2 * y[p1] - p1 as i32 + t[p1 + 1]);
        }
        ans
    }
}
