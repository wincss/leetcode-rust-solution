use crate::*;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        fn is_palindrome(s: &str) -> bool {
            let s = s.as_bytes();
            let n = s.len();
            for i in 0..n >> 1 {
                if s[i] != s[n - i - 1] {
                    return false;
                }
            }
            true
        }
        let mut ans = HashSet::new();
        let words: HashMap<String, usize> =
            words.into_iter().enumerate().map(|(x, y)| (y, x)).collect();
        for (w, &i) in words.iter() {
            let n = w.len();
            for j in 0..=n {
                if is_palindrome(&w[0..j]) {
                    let rev = w[j..n].chars().rev().collect::<String>();
                    if let Some(&k) = words.get(&rev) {
                        if i != k {
                            ans.insert(vec![k as i32, i as i32]);
                        }
                    }
                }
                if is_palindrome(&w[j..n]) {
                    let rev = w[0..j].chars().rev().collect::<String>();
                    if let Some(&k) = words.get(&rev) {
                        if i != k {
                            ans.insert(vec![i as i32, k as i32]);
                        }
                    }
                }
            }
        }
        ans.into_iter().collect::<Vec<Vec<i32>>>()
    }
}
