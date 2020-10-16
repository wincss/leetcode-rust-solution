use crate::*;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut l = 0;
        let mut r = a.len() - 1;
        let mut result = vec![];
        while l <= r {
            if a[l].abs() > a[r].abs() {
                // result.insert(0, a[l] * a[l]);
                result.push(a[l] * a[l]);
                l += 1;
            } else {
                // result.insert(0, a[r] * a[r]);
                result.push(a[r] * a[r]);
                if r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        result.reverse();
        result
    }
}
