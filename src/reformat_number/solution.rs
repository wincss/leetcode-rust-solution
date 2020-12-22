use crate::*;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let number: String = number.chars().filter(|v| *v >= '0' && *v <= '9').collect();
        let n = number.len();
        let mut p = 0;
        let mut result = String::new();
        while p < n {
            if result.len() > 0 {
                result.push('-');
            }
            match n - p {
                2 | 4 => {
                    result.push_str(&number[p..p + 2]);
                    p += 2;
                }
                _ => {
                    result.push_str(&number[p..p + 3]);
                    p += 3;
                }
            }
        }
        result
    }
}
