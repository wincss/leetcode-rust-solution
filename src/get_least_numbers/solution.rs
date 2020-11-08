use crate::*;

impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        arr.sort();
        arr.into_iter().take(k as usize).collect()
    }
}
