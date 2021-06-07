use crate::*;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let sum = stones.iter().fold(0, |s, &v| s + v) as usize;
        let mut dp = vec![false; sum / 2 + 1];
        dp[0] = true;
        for i in 0..n {
            for j in (0..=sum / 2).rev() {
                if j >= stones[i] as usize {
                    dp[j] |= dp[j - stones[i] as usize];
                }
            }
        }
        for j in (0..=sum / 2).rev() {
            if dp[j] {
                return (sum - 2 * j) as i32;
            }
        }
        unreachable!();
    }
}
