use crate::*;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let distance = target[0].abs() + target[1].abs();
        for item in ghosts.into_iter() {
            if (item[0] - target[0]).abs() + (item[1] - target[1]).abs() <= distance {
                return false;
            }
        }
        return true;
    }
}
