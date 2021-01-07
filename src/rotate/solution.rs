use crate::*;

impl Solution {
    pub fn rotate_by_reverse(nums: &mut Vec<i32>, k: i32) {
        fn reverse(nums: &mut Vec<i32>, from: usize, to: usize) {
            let mut from = from;
            let mut to = to;
            while from < to {
                let t = nums[from];
                nums[from] = nums[to];
                nums[to] = t;
                from += 1;
                to -= 1;
            }
        }
        let n = nums.len();
        if n == 0 {
            return;
        }
        let k = k as usize % n;
        if k == 0 {
            return;
        }
        reverse(nums, 0, n - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, n - 1);
    }

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
