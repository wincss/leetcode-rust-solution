use crate::*;

impl Solution {
    pub fn num_decodings_91(s: String) -> i32 {
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

    pub fn num_decodings_639(s: String) -> i32 {
        const MOD: i64 = 1000000007;
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp1 = 1;
        let mut dp2 = 0;

        for i in (0..n).rev() {
            let tmp = match (s[i], s.get(i + 1).unwrap_or(&'A').clone()) {
                ('*', '*') => dp1 * 9 + dp2 * 9 + dp2 * 6,
                ('*', x) if x <= '6' => dp1 * 9 + dp2 + dp2,
                ('*', _) => dp1 * 9 + dp2,
                ('0', _) => 0,
                ('1', '*') => dp1 + dp2 * 9,
                ('1', _) => dp1 + dp2,
                ('2', '*') => dp1 + dp2 * 6,
                ('2', x) if x <= '6' => dp1 + dp2,
                _ => dp1,
            };
            dp2 = dp1;
            dp1 = tmp % MOD;
            // println!("dp1={}, dp2={}", dp1, dp2);
        }
        dp1 as i32
    }
}
