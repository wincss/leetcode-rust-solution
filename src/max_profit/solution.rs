use crate::*;

impl Solution {
    pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
        let mut empty = 0;
        let mut full = -prices[0];
        for i in 1..prices.len() {
            let new_full = std::cmp::max(empty - prices[i], full);
            empty = std::cmp::max(full + prices[i], empty);
            full = new_full;
        }
        empty
    }
}
