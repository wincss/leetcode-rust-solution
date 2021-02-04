use crate::*;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut s = 0;
        let mut result = None;
        for (i, &v) in nums.iter().enumerate() {
            s += v;
            if i >= k {
                s -= nums[i - k];
            }
            if i >= k - 1 {
                let avg = s as f64 / k as f64;
                result = result.map(|v| avg.max(v)).or(Some(avg));
            }
        }
        result.unwrap()
    }
}
