use crate::*;

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut result = vec![];
        let mut n = 0;
        for i in a {
            n = n * 2 + i;
            result.push(n % 5 == 0);
            n = n % 10;
        }
        result
    }
}
