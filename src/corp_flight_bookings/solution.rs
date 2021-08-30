use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut b = HashMap::new();
        for item in bookings.into_iter() {
            *b.entry(item[0] - 1).or_insert(0) += item[2];
            *b.entry(item[1]).or_insert(0) -= item[2];
        }
        let mut result = vec![];
        let mut last = 0;
        for i in 0..n {
            last += *b.get(&i).unwrap_or(&0);
            result.push(last);
        }
        result
    }
}
