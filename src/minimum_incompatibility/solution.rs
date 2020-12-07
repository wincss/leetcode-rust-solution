use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let p = n / k as usize;
        fn dp(
            st: i32,
            r: usize,
            last: usize,
            n: usize,
            p: usize,
            nums: &Vec<i32>,
            saved: &mut HashMap<(i32, usize), Option<i32>>,
        ) -> Option<i32> {
            if st == 0 {
                return Some(0);
            }
            if saved.contains_key(&(st, last)) {
                return saved[&(st, last)];
            }
            let mut result = None;
            for i in last..n {
                if st & (1 << i) == 0 {
                    continue;
                }
                if last > 0 && nums[i] == nums[last - 1] {
                    continue;
                }
                if let Some(mut new) = dp(
                    st ^ (1 << i),
                    r - 1,
                    if (r - 1) % p == 0 { 0 } else { i + 1 },
                    n,
                    p,
                    nums,
                    saved,
                ) {
                    if last > 0 {
                        new += nums[i] - nums[last - 1];
                    }
                    result = result.map(|old| std::cmp::min(old, new)).or(Some(new));
                }
            }
            saved.insert((st, last), result);
            result
        }
        let mut saved = HashMap::new();
        dp((1 << n) - 1, n, 0, n, p, &nums, &mut saved).unwrap_or(-1)
    }
}
