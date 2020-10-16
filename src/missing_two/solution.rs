use crate::*;

impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() + 2;
        let mut sum = 0;
        let mut sq_sum = 0;
        for i in 1..=n {
            sum += i as i64;
            sq_sum += (i * i) as i64;
        }
        for i in nums {
            sum -= i as i64;
            sq_sum -= (i * i) as i64;
        }
        let delta = ((2 * sq_sum - sum * sum) as f64).sqrt() as i64;
        vec![((sum + delta) / 2) as i32, ((sum - delta) / 2) as i32]
    }
}
