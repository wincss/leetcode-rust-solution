use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let h = j.chars().collect::<HashSet<char>>();
        s.chars().filter(|v| h.contains(v)).count() as i32
    }
}
