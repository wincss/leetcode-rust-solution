use crate::*;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut count = [0; 10];
        let mut n = n as usize;
        while n > 0 {
            count[n % 10] += 1;
            n /= 10;
        }
        let mut power_of_2: i32 = 1;
        loop {
            println!("{}", power_of_2);
            let mut count_2 = [0; 10];
            let mut tmp = power_of_2;
            while tmp > 0 {
                count_2[(tmp % 10) as usize] += 1;
                tmp /= 10;
            }
            if (0..10).all(|idx| count[idx] == count_2[idx]) {
                return true;
            }
            if let Some(v) = power_of_2.checked_mul(2) {
                power_of_2 = v;
            } else {
                break;
            }
        }
        false
    }
}
