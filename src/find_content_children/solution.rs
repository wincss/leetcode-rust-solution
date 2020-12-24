use crate::*;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort();
        s.sort();
        let mut result = 0;
        let mut idx = 0;
        for i in s {
            if i >= g[idx] {
                result += 1;
                idx += 1;
            }
            if idx == g.len() {
                break;
            }
        }
        result
    }
}
