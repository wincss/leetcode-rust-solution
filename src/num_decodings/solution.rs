use crate::*;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp1 = 1;
        let mut dp2 = 0;
        for i in (0..n).rev() {
            let tmp = match s[i] {
                '0' => 0,
                '1' => dp1 + dp2,
                '2' if i < n - 1 && s[i + 1] <= '6' => dp1 + dp2,
                _ => dp1,
            };
            dp2 = dp1;
            dp1 = tmp;
            // println!("dp1={}, dp2={}", dp1, dp2);
        }
        dp1
    }
}
