use crate::*;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        assert!(num >= 1 && num <= 3999);

        let one = vec!['I', 'X', 'C', 'M'];
        let five = vec!['V', 'L', 'D'];
        let mut result = String::new();
        let mut base = 1000;
        for idx in (0_usize..4).rev() {
            let digit = (num / base % 10) as usize;
            base /= 10;
            match digit {
                v @ 0..=3 => {
                    for _ in 0..v {
                        result.push(one[idx]);
                    }
                }
                4 => {
                    result.push(one[idx]);
                    result.push(five[idx])
                }
                v @ 5..=8 => {
                    result.push(five[idx]);
                    for _ in 0..v - 5 {
                        result.push(one[idx]);
                    }
                }
                9 => {
                    result.push(one[idx]);
                    result.push(one[idx + 1]);
                }
                _ => unreachable!(),
            };
        }
        result
    }
}
