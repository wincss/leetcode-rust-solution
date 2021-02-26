use crate::*;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let words: Vec<i32> = words
            .iter()
            .map(|v| v.chars().fold(0, |acc, c| acc | 1 << (c as u8 - 97)))
            .collect();
        let mut result = vec![];
        for s in puzzles.iter() {
            let c = s.chars().next().unwrap() as u8 - 97;
            let bits = s.chars().fold(0, |acc, c| acc | 1 << (c as u8 - 97));
            let mut count = 0;
            for &w in words.iter() {
                if w & (1 << c) > 0 && w & (!bits) == 0 {
                    count += 1;
                }
            }
            result.push(count);
        }
        result
    }
}
