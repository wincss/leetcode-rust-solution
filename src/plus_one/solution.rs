use crate::*;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        for v in digits.iter_mut().rev() {
            *v += carry;
            carry = *v / 10;
            *v %= 10;
        }
        if carry > 0 {
            digits.insert(0, carry);
        }
        digits
    }
}
