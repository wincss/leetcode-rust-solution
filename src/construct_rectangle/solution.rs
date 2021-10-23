use crate::*;
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let sqrt = (area as f64).sqrt() as i32;
        let width = (1..=sqrt).rev().filter(|&x| area % x == 0).nth(0).unwrap();
        vec![area / width, width]
    }
}
