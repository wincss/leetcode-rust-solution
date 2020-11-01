use crate::*;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn wb(
            s: &str,
            word_len: &HashSet<usize>,
            word_dict: &HashSet<String>,
            saved: &mut HashMap<usize, Vec<Vec<String>>>,
        ) -> Vec<Vec<String>> {
            if s.len() == 0 {
                return vec![vec![]];
            }
            if saved.contains_key(&s.len()) {
                return saved[&s.len()].clone();
            }
            let mut result = vec![];
            for &len in word_len.iter() {
                if len > s.len() {
                    continue;
                }
                if word_dict.contains(&s[..len]) {
                    for mut word_set in wb(&s[len..], word_len, word_dict, saved) {
                        word_set.insert(0, String::from(&s[..len]));
                        result.push(word_set);
                    }
                }
            }
            saved.insert(s.len(), result.clone());
            result
        }
        let mut saved = HashMap::new();
        let word_len: HashSet<usize> = word_dict.iter().map(|v| v.len()).collect();
        let word_dict: HashSet<String> = word_dict.into_iter().collect();
        wb(&s[..], &word_len, &word_dict, &mut saved)
            .into_iter()
            .map(|v| v.join(" "))
            .collect()
    }
}
