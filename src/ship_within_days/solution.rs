use crate::*;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut l = 0;
        let mut r = 0;
        for &i in weights.iter() {
            l = l.max(i);
            r += i;
        }
        while l < r {
            let m = (l + r) / 2;
            let mut k = 0;
            let mut w = 0;
            for &i in weights.iter() {
                if w + i > m {
                    w = i;
                    k += 1;
                } else {
                    w += i;
                }
            }
            if w > 0 {
                k += 1;
            }
            if k > d {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}
