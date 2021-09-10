use crate::*;
use std::collections::HashMap;

impl Solution {
    pub fn find_integers(num: i32) -> i32 {
        let n = 32 - num.leading_zeros() as usize;
        fn dp(
            k: usize,
            n: usize,
            num: i32,
            ubound: bool,
            continuous_ones: u8,
            save: &mut HashMap<(usize, bool, u8), i32>,
        ) -> i32 {
            if k == n {
                return if continuous_ones == 2 { 0 } else { 1 };
            }
            if save.contains_key(&(k, ubound, continuous_ones)) {
                return save[&(k, ubound, continuous_ones)];
            }
            let mut result = 0;
            let h = if ubound && num & (1 << (n - 1 - k)) == 0 {
                0
            } else {
                1
            };
            for i in 0..=h {
                let ubound = ubound && (i == h);
                let continuous_ones = match (continuous_ones, i) {
                    (2, _) => 2,
                    (1, 1) => 2,
                    (0, 1) => 1,
                    _ => 0,
                };
                result += dp(k + 1, n, num, ubound, continuous_ones, save);
            }
            save.insert((k, ubound, continuous_ones), result);
            result
        }
        let mut save = HashMap::new();
        dp(0, n, num, true, 0, &mut save)
    }
}
