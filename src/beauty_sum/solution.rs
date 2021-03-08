use crate::*;

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let s: Vec<usize> = s.chars().map(|v| v as u8 as usize - 97).collect();
        let mut v = [0; 26];
        let mut ss = vec![];
        ss.push(v.clone());
        for &c in s.iter() {
            v[c] += 1;
            ss.push(v.clone());
        }
        let n = s.len();
        let mut result = 0;
        for left in 0..n {
            for right in left..n {
                result += (0..26)
                    .map(|v| ss[right + 1][v] - ss[left][v])
                    .max()
                    .unwrap()
                    - (0..26)
                        .map(|v| ss[right + 1][v] - ss[left][v])
                        .filter(|v| *v > 0)
                        .min()
                        .unwrap();
            }
        }
        result
    }
}
