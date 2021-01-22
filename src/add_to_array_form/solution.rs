use crate::*;

impl Solution {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut idx = a.len();
        let mut carry = k;
        let mut a = a;
        while carry > 0 {
            if idx == 0 {
                a.insert(0, 0);
            } else {
                idx -= 1;
            }
            let d = a[idx] + carry;
            a[idx] = d % 10;
            carry = d / 10;
        }
        a
    }
}
