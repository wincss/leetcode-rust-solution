use crate::*;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number;
        let mut s = String::new();
        while n > 0 {
            let d = n % 26;
            n = n / 26;
            if d == 0 {
                n -= 1;
                s.insert(0, 'Z');
            } else {
                s.insert(0, (d + 64) as u8 as char);
            }
        }
        s
    }
}
