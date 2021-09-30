use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut d = HashMap::new();
        for mut path in paths.into_iter() {
            d.entry(path.pop().unwrap()).or_insert(0);
            *d.entry(path.pop().unwrap()).or_insert(0) += 1;
        }
        for (city, degree) in d.into_iter() {
            if degree == 0 {
                return city;
            }
        }
        unreachable!();
    }
}
