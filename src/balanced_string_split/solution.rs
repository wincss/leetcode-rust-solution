use crate::*;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut count = 0;
        for c in s.chars() {
            match c {
                'L' => l += 1,
                'R' => r += 1,
                _ => unreachable!(),
            }
            if l == r {
                count += 1;
                l = 0;
                r = 0;
            }
        }
        count
    }
}
