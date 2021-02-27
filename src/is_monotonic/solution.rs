use crate::*;

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let n = a.len();
        let mut dir = 0;
        for i in 1..n {
            match (a[i] - a[i - 1]).signum() {
                0 => continue,
                v if dir != 0 && v != dir => return false,
                v => dir = v,
            }
        }
        true
    }
}
