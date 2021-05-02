use crate::*;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut x = x;
        while x != 0 {
            if let Some(v) = ans.checked_mul(10).and_then(|v| v.checked_add(x % 10)) {
                x = x / 10;
                ans = v;
            } else {
                return 0;
            }
        }
        ans
    }
}
