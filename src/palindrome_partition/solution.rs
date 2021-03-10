use crate::*;

use std::collections::HashSet;
impl Solution {
    // partition in origin problem
    pub fn palindrome_partition(s: String) -> Vec<Vec<String>> {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut palindrome_end = vec![HashSet::new(); n];
        for length in 1..=n {
            for start in 0..=n - length {
                match length {
                    1 => {
                        palindrome_end[start].insert(start + length);
                    }
                    2 if s[start] == s[start + 1] => {
                        palindrome_end[start].insert(start + length);
                    }
                    _ if s[start] == s[start + length - 1]
                        && palindrome_end[start + 1].contains(&(start + length - 1)) =>
                    {
                        palindrome_end[start].insert(start + length);
                    }
                    _ => {}
                }
            }
        }
        fn dfs(
            start: usize,
            n: usize,
            s: &Vec<char>,
            palindrome_end: &Vec<HashSet<usize>>,
            current: &mut Vec<String>,
            result: &mut Vec<Vec<String>>,
        ) {
            if start == n {
                result.push(current.clone());
                return;
            }
            for &i in palindrome_end[start].iter() {
                current.push(s[start..i].iter().collect());
                dfs(i, n, s, palindrome_end, current, result);
                current.pop();
            }
        }
        let mut result = vec![];
        let mut current = vec![];
        dfs(0, n, &s, &palindrome_end, &mut current, &mut result);
        result
    }
}
