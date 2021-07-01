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

    pub fn max_ice_cream_by_counting_sort(costs: Vec<i32>, coins: i32) -> i32 {
        const MAX_COST: usize = 100000;
        let mut count = vec![0; MAX_COST + 1];
        for i in costs {
            count[i as usize] += 1;
        }
        let mut remain = coins as usize;
        let mut amount = 0;
        for i in 1..=MAX_COST {
            if count[i] == 0 {
                continue;
            }
            let current = count[i].min(remain / i);
            amount += current;
            remain -= i * current;
            if current < count[i] {
                break;
            }
        }
        amount as i32
    }
}
