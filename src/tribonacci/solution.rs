use crate::*;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut t = vec![0, 1, 1];
        if n < 3 {
            t[n as usize]
        } else {
            for _ in 3..=n {
                let v = t[0] + t[1] + t[2];
                t[0] = t[1];
                t[1] = t[2];
                t[2] = v;
            }
            t[2]
        }
    }
}
