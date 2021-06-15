use crate::*;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        arr.into_iter()
            .enumerate()
            .max_by_key(|(_, value)| *value)
            .unwrap()
            .0 as i32
    }
}
