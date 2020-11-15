use crate::*;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_by_key(|v| (-v[0], v[1]));
        // println!("{:?}", people);
        let mut result = vec![];
        for i in people.into_iter() {
            result.insert(i[1] as usize, i);
        }
        result
    }
}
