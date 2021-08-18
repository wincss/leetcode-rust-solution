use crate::*;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = vec![];
        for c in s.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => vowels.push(c),
                _ => {}
            }
        }
        let mut result = String::new();
        for c in s.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    result.push(vowels.pop().unwrap())
                }
                _ => result.push(c),
            }
        }
        result
    }
}
