use crate::*;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        const BASE: i32 = 16;
        const LENGTH: usize = 8;
        const HEX_CHARS: [char; BASE as usize] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        if num == 0 {
            return "0".to_string();
        }
        (0..LENGTH)
            .scan(num, |num, _| {
                if *num == 0 {
                    None
                } else {
                    let digit = num.rem_euclid(BASE) as usize;
                    *num = num.div_euclid(BASE);
                    Some(HEX_CHARS[digit])
                }
            })
            .fuse()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }
}
