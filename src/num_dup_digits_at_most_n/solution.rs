use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let mut n = n;
        let mut digits = vec![];
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        let n = digits.len();
        fn dp(
            k: usize,
            n: usize,
            digits: &Vec<i32>,
            ubound: bool,
            state: i32,
            save: &mut HashMap<(usize, bool, i32), i32>,
        ) -> i32 {
            if k == n {
                return if state == -1 { 1 } else { 0 };
            }
            if save.contains_key(&(k, ubound, state)) {
                return save[&(k, ubound, state)];
            }
            let mut result = 0;
            let h = if ubound { digits[k] } else { 9 };
            for i in 0..=h {
                let ubound = ubound && (i == h);
                let state = if i == 0 && state == 0 {
                    state
                } else if state & (1 << i) > 0 {
                    -1
                } else {
                    state | (1 << i)
                };
                result += dp(k + 1, n, digits, ubound, state, save);
            }
            save.insert((k, ubound, state), result);
            result
        }
        let mut save = HashMap::new();
        dp(0, n, &digits, true, 0, &mut save)
    }
}
