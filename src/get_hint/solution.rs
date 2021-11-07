use crate::*;
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut secret: Vec<char> = secret.chars().collect();
        let mut guess: Vec<char> = guess.chars().collect();
        let n = secret.len();
        let mut a = 0;
        let mut b = 0;
        for i in 0..n {
            if secret[i] == guess[i] {
                a += 1;
            }
        }
        secret.sort();
        guess.sort();
        let mut idx = 0;
        for i in 0..n {
            while idx < n && secret[i] > guess[idx] {
                idx += 1;
            }
            if idx < n && secret[i] == guess[idx] {
                b += 1;
                idx += 1;
            }
        }
        format!("{}A{}B", a, b - a)
    }
}
