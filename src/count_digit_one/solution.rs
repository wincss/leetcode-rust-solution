use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn number_of2s_in_range(n: i32) -> i32 {
        Self::count_digit(n, 2)
    }
    pub fn count_digit_one(n: i32) -> i32 {
        Self::count_digit(n, 1)
    }
    fn count_digit(n: i32, digit: i32) -> i32 {
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
            digit: i32,
            digits: &Vec<i32>,
            ubound: bool,
            state: i32,
            save: &mut HashMap<(usize, bool, i32), i32>,
        ) -> i32 {
            if k == n {
                return state;
            }
            if save.contains_key(&(k, ubound, state)) {
                return save[&(k, ubound, state)];
            }
            let mut result = 0;
            let h = if ubound { digits[k] } else { 9 };
            for i in 0..=h {
                let ubound = ubound && (i == h);
                let state = if i == digit { state + 1 } else { state };
                result += dp(k + 1, n, digit, digits, ubound, state, save);
            }
            save.insert((k, ubound, state), result);
            result
        }
        let mut save = HashMap::new();
        dp(0, n, digit, &digits, true, 0, &mut save)
    }
}
