use crate::*;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut chars = "123456789".chars().collect::<Vec<char>>();
        let mut result = String::new();
        let mut k = k - 1;
        let mut s = 1;
        for i in 2..=n {
            s *= i;
        }
        for i in (1..=n).rev() {
            s /= i;
            result.push(chars.remove((k / s) as usize));
            k = k % s;
        }
        result
    }
}
