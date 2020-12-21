use crate::*;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut c2 = 0;
        let mut c1 = 0;
        for i in (0..n).rev() {
            let c = std::cmp::min(c1, c2) + cost[i];
            c2 = c1;
            c1 = c;
        }
        std::cmp::min(c1, c2)
    }
}
