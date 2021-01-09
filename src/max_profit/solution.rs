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

    pub fn max_profit_iii(prices: Vec<i32>) -> i32 {
        Self::max_profit_iv(2, prices)
    }

    pub fn max_profit_iv(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 {
            return 0;
        }
        let k = std::cmp::min(k as usize, n / 2);
        let mut empty = vec![0; k + 1];
        let mut full = vec![-prices[0]; k + 1];
        full[k] = std::i32::MIN;
        let mut result = 0;
        for i in 1..n {
            for j in (0..=k).rev() {
                let new_empty = std::cmp::max(empty[j], prices[i] + full[j]);
                if j < k {
                    full[j] = std::cmp::max(full[j], -prices[i] + empty[j + 1]);
                }
                empty[j] = new_empty;
                result = std::cmp::max(result, new_empty);
            }
        }
        // println!("prices={:?}", prices);
        // println!("empty={:?}, full={:?}", empty, full);
        result
    }
}
