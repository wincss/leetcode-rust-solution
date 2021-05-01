use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut space = HashMap::new();
        let mut last = 0;
        for row in wall.iter() {
            last = 0;
            for &i in row.iter() {
                last += i;
                *space.entry(last).or_insert(0) += 1;
            }
        }
        space.remove(&last);
        wall.len() as i32 - *space.values().max().unwrap_or(&0)
    }
}
