use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        fn dp(
            k: usize,
            last_color: i32,
            left_block: i32,
            saved: &mut HashMap<(usize, i32, i32), Option<i32>>,
            houses: &Vec<i32>,
            cost: &Vec<Vec<i32>>,
            m: i32,
            n: i32,
        ) -> Option<i32> {
            if left_block < 0 {
                return None;
            }
            if k == m as usize {
                return if left_block != 0 { None } else { Some(0) };
            }
            let key = (k, last_color, left_block);
            if let Some(v) = saved.get(&key) {
                return v.clone();
            }
            if houses[k] > 0 {
                let v = dp(
                    k + 1,
                    houses[k],
                    if houses[k] == last_color {
                        left_block
                    } else {
                        left_block - 1
                    },
                    saved,
                    houses,
                    cost,
                    m,
                    n,
                );
                saved.insert(key, v.clone());
                return v;
            }
            let mut min_cost: Option<i32> = None;
            for i in 1..=n {
                if let Some(v) = dp(
                    k + 1,
                    i,
                    if i == last_color {
                        left_block
                    } else {
                        left_block - 1
                    },
                    saved,
                    houses,
                    cost,
                    m,
                    n,
                ) {
                    let new = cost[k][i as usize - 1] + v;
                    min_cost = min_cost.map(|old| old.min(new)).or(Some(new));
                }
            }
            saved.insert(key, min_cost.clone());
            min_cost
        }
        let mut saved = HashMap::new();
        dp(0, 0, target, &mut saved, &houses, &cost, m, n).unwrap_or(-1)
    }
}
