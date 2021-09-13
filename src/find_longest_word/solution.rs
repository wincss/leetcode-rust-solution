use crate::*;

impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut longest = 0;
        let mut longest_word = String::new();
        let mut dictionary = dictionary;
        dictionary.sort();
        for st in dictionary.into_iter() {
            let mut ok = true;
            let mut p = 0;
            for c in st.chars() {
                while p < n && c != s[p] {
                    p += 1
                }
                if p == n {
                    ok = false;
                    break;
                }
                p += 1;
            }
            if ok && st.len() > longest {
                longest = st.len();
                longest_word = st;
            }
        }
        longest_word
    }
}
