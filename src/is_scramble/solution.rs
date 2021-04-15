use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        assert!(n == s2.len());

        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();

        let mut h = vec![0; 26];
        for &c in s1.iter() {
            h[c as u8 as usize - 97] += 1;
        }
        for &c in s2.iter() {
            h[c as u8 as usize - 97] -= 1;
        }
        if h != vec![0; 26] {
            return false;
        }

        fn is_scramble_str<'a>(
            s1: &'a [char],
            s2: &'a [char],
            saved: &mut HashMap<(&'a [char], &'a [char]), bool>,
        ) -> bool {
            // println!("{:?}, {:?}", s1, s2);
            if s1 == s2 {
                return true;
            }
            let key = (s1, s2);
            if saved.contains_key(&key) {
                return saved[&key];
            }
            let n = s1.len();
            let mut h1 = vec![0; 26];
            let mut h2 = vec![0; 26];
            let mut h3 = vec![0; 26];

            for i in 1..n {
                h1[s1[i - 1] as u8 as usize - 97] += 1;
                h2[s2[i - 1] as u8 as usize - 97] += 1;
                h3[s2[n - i] as u8 as usize - 97] += 1;
                if h1 == h2 {
                    if is_scramble_str(&s1[0..i], &s2[0..i], saved)
                        && is_scramble_str(&s1[i..n], &s2[i..n], saved)
                    {
                        saved.insert(key, true);
                        return true;
                    }
                }
                if h1 == h3 {
                    if is_scramble_str(&s1[0..i], &s2[n - i..n], saved)
                        && is_scramble_str(&s1[i..n], &s2[0..n - i], saved)
                    {
                        saved.insert(key, true);
                        return true;
                    }
                }
            }
            saved.insert(key, false);
            false
        }
        let mut saved = HashMap::new();
        is_scramble_str(&s1[..], &s2[..], &mut saved)
    }
}
