use crate::*;

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        costs.sort();
        let mut cost = 0;
        for (i, &v) in costs.iter().enumerate() {
            cost += v;
            if cost > coins {
                return i as i32;
            }
        }
        costs.len() as i32
    }
}
