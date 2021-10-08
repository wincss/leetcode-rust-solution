use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        const MAP: [char; 4] = ['A', 'T', 'G', 'C'];
        let mut d = HashMap::new();
        let mut last = 0;
        for (idx, c) in s.char_indices() {
            last = (last << 2) & 0xfffff
                | match c {
                    'A' => 0,
                    'T' => 1,
                    'G' => 2,
                    'C' => 3,
                    _ => unreachable!(),
                };
            if idx >= 9 {
                *d.entry(last).or_insert(0) += 1;
            }
        }
        let mut result = vec![];
        for (key, value) in d.into_iter() {
            if value > 1 {
                result.push(
                    (0..10)
                        .rev()
                        .map(|mask| MAP[(key >> (2 * mask)) & 3])
                        .collect(),
                );
            }
        }
        result
    }
}
