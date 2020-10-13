use crate::*;

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut intersection = vec![std::i32::MAX; 26];
        for i in a.into_iter() {
            let mut count = vec![0; 26];
            for &j in i.as_bytes() {
                count[j as usize - 97] += 1;
            }
            for j in 0..26 {
                intersection[j] = std::cmp::min(intersection[j], count[j]);
            }
        }
        let mut result = vec![];
        for i in 0..26 {
            for _ in 0..intersection[i] {
                result.push(((i + 97) as u8 as char).to_string());
            }
        }
        result
    }
}
