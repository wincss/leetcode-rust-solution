use crate::*;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 0 {
            return;
        }
        let k = k as usize % n;
        if k == 0 {
            return;
        }
        let mut remaining = n;
        let mut start = 0;
        while remaining > 0 {
            let mut last = Some(0);
            let mut idx = start;
            loop {
                nums[idx] = last.replace(nums[idx]).unwrap();
                remaining -= 1;
                idx = (idx + k) % n;
                if idx == start {
                    break;
                }
            }
            nums[idx] = last.unwrap();
            start += 1;
        }
    }
}
