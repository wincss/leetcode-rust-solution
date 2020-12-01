use crate::*;

impl Solution {
    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        fn f(s: &String) -> usize {
            let mut minc = 'z';
            let mut minv = 0;
            for i in s.chars() {
                if i == minc {
                    minv += 1;
                } else if i < minc {
                    minc = i;
                    minv = 1;
                }
            }
            minv
        }
        let max_length = words.iter().map(|v| v.len()).max().unwrap();
        let mut acc = vec![0; max_length + 2];
        for w in words.iter() {
            acc[f(w)] += 1;
        }
        for i in (0..=max_length).rev() {
            acc[i] += acc[i + 1];
        }
        queries.iter().map(|v| acc[f(v) + 1]).collect()
    }
}
