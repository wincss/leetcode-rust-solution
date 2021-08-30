use crate::*;

impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        const MOD: i64 = 1000000007;
        let mut ending0 = 0;
        let mut ending1 = 0;
        let mut has0 = 0;
        for c in binary.chars() {
            match c {
                '1' => ending1 = (ending0 + ending1 + 1) % MOD,
                '0' => {
                    ending0 = (ending0 + ending1) % MOD;
                    has0 = 1;
                }
                _ => unreachable!(),
            }
        }
        ((ending0 + ending1 + has0) % MOD) as i32
    }
}
