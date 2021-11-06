use crate::*;
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut minm = m;
        let mut minn = n;
        for item in ops.iter() {
            minm = minm.min(item[0]);
            minn = minn.min(item[1]);
        }
        minm * minn
    }
}
