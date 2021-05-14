use crate::*;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut last = 0;
        for i in s.chars() {
            last = match i {
                'I' => 1,
                'V' if last == 1 => 3,
                'V' => 5,
                'X' if last == 1 => 8,
                'X' => 10,
                'L' if last == 10 => 30,
                'L' => 50,
                'C' if last == 10 => 80,
                'C' => 100,
                'D' if last == 100 => 300,
                'D' => 500,
                'M' if last == 100 => 800,
                'M' => 1000,
                _ => unreachable!(),
            };
            result += last;
        }
        result
    }
}
