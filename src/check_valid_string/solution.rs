use crate::*;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut max_depth = 0;
        let mut min_depth = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    max_depth += 1;
                    min_depth += 1;
                }
                ')' => {
                    max_depth -= 1;
                    if max_depth < 0 {
                        return false;
                    }
                    min_depth = 0.max(min_depth - 1);
                }
                '*' => {
                    max_depth += 1;
                    min_depth = 0.max(min_depth - 1);
                }
                _ => unreachable!(),
            }
        }
        min_depth == 0
    }
}
