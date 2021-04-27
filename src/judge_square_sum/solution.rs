use crate::*;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut l = 0;
        let mut r = (c as f64).sqrt() as i32;
        while l <= r {
            if let Some(v) = r.checked_mul(r).and_then(|v| v.checked_add(l * l)) {
                if v == c {
                    return true;
                } else if v < c {
                    l += 1;
                } else {
                    r -= 1;
                }
            } else {
                r -= 1;
            }
        }
        false
    }
}
