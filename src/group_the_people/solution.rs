use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut people_by_group_size = HashMap::new();
        for (i, v) in group_sizes.into_iter().enumerate() {
            people_by_group_size
                .entry(v as usize)
                .or_insert(vec![])
                .push(i as i32);
        }
        let mut result = vec![];
        for (size, mut people) in people_by_group_size.into_iter() {
            while people.len() > 0 {
                result.push(people.drain(0..size).collect());
            }
        }
        result
    }
}
