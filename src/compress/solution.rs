use crate::*;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut rp = 0;
        let mut wp = 0;
        let mut last = 'a';
        let mut count = 0;

        while rp <= n {
            if rp < n && chars[rp] == last {
                count += 1;
                rp += 1;
                continue;
            }
            if count > 0 {
                chars[wp] = last;
                wp += 1;
                if count > 1 {
                    for c in count.to_string().chars() {
                        chars[wp] = c;
                        wp += 1;
                    }
                }
            }
            if rp < n {
                count = 1;
                last = chars[rp];
            }
            // println!("{:?} {} {}", chars, rp, wp);
            rp += 1;
        }
        // println!("{:?} {} {}", chars, rp, wp);
        wp as i32
    }
}
