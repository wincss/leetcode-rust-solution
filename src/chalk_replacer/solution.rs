use crate::*;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut s = 0;
        for (idx, &v) in chalk.iter().enumerate() {
            if k < v {
                return idx as i32;
            }
            s += v;
            k -= v;
        }
        k = k % s;
        for (idx, &v) in chalk.iter().enumerate() {
            if k < v {
                return idx as i32;
            }
            k -= v;
        }
        unreachable!();
    }
}
