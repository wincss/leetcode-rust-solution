use crate::*;

use std::{collections::HashMap, str::Chars};

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        fn count(
            result: &mut HashMap<String, usize>,
            last: &mut HashMap<String, usize>,
            last_string: &mut String,
            last_number: &mut usize,
        ) {
            if *last_number == 0 {
                *last_number = 1;
            }
            if !last_string.is_empty() {
                assert!(last.is_empty());
                *result.entry(last_string.clone()).or_insert(0) += *last_number;
                *last_string = String::new();
            } else {
                assert!(last_string.is_empty());
                for (key, value) in last.drain() {
                    *result.entry(key).or_insert(0) += value * *last_number;
                }
            }
            *last_number = 0;
        }
        fn parse(s: &mut Chars) -> HashMap<String, usize> {
            let mut result = HashMap::new();
            let mut last: HashMap<String, usize> = HashMap::new();
            let mut last_string = String::new();
            let mut last_number = 0;
            while let Some(c) = s.next() {
                if c == '(' || c == ')' || (c >= 'A' && c <= 'Z') {
                    count(&mut result, &mut last, &mut last_string, &mut last_number);
                }
                match c {
                    '(' => last = parse(s),
                    ')' => break,
                    'A'..='Z' | 'a'..='z' => last_string.push(c),
                    '0'..='9' => last_number = last_number * 10 + c as u8 as usize - 48,
                    _ => unreachable!(),
                }
            }
            count(&mut result, &mut last, &mut last_string, &mut last_number);
            // println!("{:?}", result);
            result
        }
        let mut iter = formula.chars();
        let count = parse(&mut iter);
        let mut keys: Vec<(String, usize)> = count.into_iter().collect();
        keys.sort();
        let mut result = String::new();
        for (key, value) in keys.into_iter() {
            result.push_str(&key);
            if value > 1 {
                result.push_str(&value.to_string());
            }
        }
        result
    }
}
