use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut position = HashMap::new();
        for (i, v) in arr2.into_iter().enumerate() {
            position.insert(v, i as i32);
        }
        let mut arr1 = arr1;
        arr1.sort_by_key(|v| position.get(v).map_or(*v, |&pos| std::i32::MIN + pos));
        arr1
    }
}
