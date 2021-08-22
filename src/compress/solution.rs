use crate::*;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut rp = 0;
        let mut wp = 0;
        let mut last = 'a';
        let mut count = 0;

        while rp < n {
            if chars[rp] == last {
                count += 1;
                rp += 1;
                continue;
            }
            if count > 0 {
                let length = if count >= 1000 {
                    4
                } else if count >= 100 {
                    3
                } else if count >= 10 {
                    2
                } else if count >= 2 {
                    1
                } else {
                    0
                };
                chars[wp] = last;
                for i in (wp + 1..=wp + length).rev() {
                    chars[i] = (count % 10 + 48) as u8 as char;
                    count /= 10;
                }
                wp += 1 + length;
            }
            count = 1;
            last = chars[rp];
            // println!("{:?} {} {}", chars, rp, wp);
            rp += 1;
        }
        if count > 0 {
            let length = if count >= 1000 {
                4
            } else if count >= 100 {
                3
            } else if count >= 10 {
                2
            } else if count >= 2 {
                1
            } else {
                0
            };
            chars[wp] = last;
            for i in (wp + 1..=wp + length).rev() {
                chars[i] = (count % 10 + 48) as u8 as char;
                count /= 10;
            }
            wp += 1 + length;
        }
        // println!("{:?} {} {}", chars, rp, wp);
        wp as i32
    }
}
