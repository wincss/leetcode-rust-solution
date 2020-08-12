use crate::*;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let l1 = num1.len();
        let l2 = num2.len();
        let num1: Vec<u8> = num1.as_bytes().iter().rev().map(|&v| v - 48).collect();
        let num2: Vec<u8> = num2.as_bytes().iter().rev().map(|&v| v - 48).collect();
        let mut ans = vec![0; l1 + l2];
        for (i, v1) in num1.into_iter().enumerate() {
            for (j, &v2) in num2.iter().enumerate() {
                ans[i + j] += (v1 * v2) as i32;
            }
        }
        let mut carry = 0;
        for i in ans.iter_mut() {
            let m = *i + carry;
            *i = m % 10;
            carry = m / 10;
        }
        assert_eq!(carry, 0);

        ans.into_iter()
            .enumerate()
            .rev()
            .skip_while(|&(i, v)| i > 0 && v == 0)
            .map(|(_, v)| (v + 48) as u8 as char)
            .collect()
    }
}
