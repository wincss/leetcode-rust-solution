use crate::*;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut curr = 0;
        let mut total = 0;
        let mut start = 0;
        for i in 0..n {
            curr += gas[i] - cost[i];
            total += gas[i] - cost[i];
            if curr < 0 {
                start = i + 1;
                curr = 0;
            }
        }
        if total < 0 {
            -1
        } else {
            start as i32
        }
    }
}
