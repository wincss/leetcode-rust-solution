use crate::*;

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let sum: i32 = machines.iter().map(|&v| v).sum();
        let n = machines.len();
        if sum % (n as i32) != 0 {
            return -1;
        }
        let avg = sum / (n as i32);
        let mut machines = machines;
        let mut result = -1;
        let mut sum = 0;
        for i in 0..n {
            machines[i] -= avg;
            sum += machines[i];
            result = result.max(sum.abs()).max(machines[i]);
        }
        result
    }
}
