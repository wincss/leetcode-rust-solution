use crate::*;
use std::collections::HashMap;

impl Solution {
    pub fn find_integers(num: i32) -> i32 {
        let mut n = num;
        let mut digits = vec![];
        while n > 0 {
            digits.push(n & 1);
            n >>= 1;
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
                return if state == 2 { 0 } else { 1 };
            }
            if save.contains_key(&(k, ubound, state)) {
                return save[&(k, ubound, state)];
            }
            let mut result = 0;
            let h = if ubound { digits[k] } else { 1 };
            for i in 0..=h {
                let ubound = ubound && (i == h);
                let state = if state == 2 {
                    2
                } else if state == 1 && i == 1 {
                    2
                } else if state == 0 && i == 1 {
                    1
                } else {
                    0
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
