use crate::*;

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 {
            return false;
        }
        let n = a.len();
        let mut dir = 1;
        for i in 1..n {
            if dir == 1 {
                if a[i] < a[i - 1] {
                    if i > 1 {
                        dir = 2;
                    } else {
                        return false;
                    }
                } else if a[i] == a[i - 1] {
                    return false;
                }
            } else {
                if a[i] >= a[i - 1] {
                    return false;
                }
            }
        }
        dir == 2
    }
}
