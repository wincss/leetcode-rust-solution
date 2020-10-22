use crate::*;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.chars().map(|v| v as u8 - 48).rev();
        let mut b = b.chars().map(|v| v as u8 - 48).rev();
        let mut carry = 0;
        let mut result = vec![];
        loop {
            let p = a.next();
            let q = b.next();
            if p.is_none() && q.is_none() && carry == 0 {
                break;
            }
            carry += p.unwrap_or(0) + q.unwrap_or(0);
            result.push(carry & 1);
            carry >>= 1;
        }
        result.into_iter().rev().map(|v| (v + 48) as char).collect()
    }
}
