use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut bigger = HashMap::new();
        let mut st = vec![];
        for &i in nums2.iter().rev() {
            while !st.is_empty() && *st.last().unwrap() < i {
                st.pop();
            }
            bigger.insert(i, *st.last().unwrap_or(&(-1)));
            st.push(i);
        }
        nums1.into_iter().map(|v| bigger[&v]).collect()
    }
}
