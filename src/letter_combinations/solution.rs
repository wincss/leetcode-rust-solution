use crate::*;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let letter_map = [
            (b'a', 3),
            (b'd', 3),
            (b'g', 3),
            (b'j', 3),
            (b'm', 3),
            (b'p', 4),
            (b't', 3),
            (b'w', 4),
        ];
        let digits = digits.as_bytes();
        let mut amount: i32 = 1;
        for &i in digits.iter() {
            amount *= letter_map[(i - 0x32) as usize].1;
        }
        let mut ans = vec![];
        for i in 0..amount {
            let mut p = i;
            let mut word = String::new();
            for &j in digits.iter() {
                let (ch, l) = letter_map[(j - 0x32) as usize];
                word.push((ch as i32 + p % l) as u8 as char);
                p /= l;
            }
            ans.push(word);
        }
        ans
    }
}
