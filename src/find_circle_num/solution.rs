use crate::*;

use std::collections::HashMap;

use common::algorithms::union_find::*;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let n = is_connected.len();
        let mut result = n;
        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 1 {
                    if union(&i, &j, &mut parent, &mut size) {
                        result -= 1;
                    }
                }
            }
        }
        result as i32
    }
}
