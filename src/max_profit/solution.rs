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

    pub fn max_profit_iv(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 {
            return 0;
        }
        let k = std::cmp::min(k as usize, n / 2);
        let mut empty = vec![vec![0; k + 1]; n];
        let mut full = vec![vec![-prices[0]; k + 1]; n];
        full[0][k] = std::i32::MIN;
        let mut result = 0;
        for i in 1..n {
            for j in (0..=k).rev() {
                empty[i][j] = std::cmp::max(empty[i - 1][j], prices[i] + full[i - 1][j]);
                if j == k {
                    full[i][j] = full[i - 1][j];
                } else {
                    full[i][j] = std::cmp::max(full[i - 1][j], -prices[i] + empty[i - 1][j + 1]);
                }
                result = std::cmp::max(result, empty[i][j]);
            }
        }
        // println!("prices={:?}", prices);
        // println!("empty={:?}, full={:?}", empty, full);
        result
    }
}
