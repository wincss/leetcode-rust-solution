use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut h = HashMap::new();
        for answer in answers.into_iter() {
            *h.entry(answer).or_insert(0) += 1;
        }
        let mut num = 0;
        for (answer, count) in h.into_iter() {
            num += ((count - 1) / (answer + 1) + 1) * (answer + 1);
        }
        num
    }
}
