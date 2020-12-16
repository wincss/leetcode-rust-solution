use crate::*;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut buyprice = std::i32::MAX;
        for i in prices {
            if i < buyprice {
                buyprice = i;
            }
            max = std::cmp::max(max, i - buyprice);
        }
        max
    }

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

    pub fn max_profit_with_fee(prices: Vec<i32>, fee: i32) -> i32 {
        let mut empty = 0;
        let mut full = -prices[0];
        for i in 1..prices.len() {
            let new_full = std::cmp::max(empty - prices[i], full);
            empty = std::cmp::max(full + prices[i] - fee, empty);
            full = new_full;
        }
        empty
    }
}
