use crate::*;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut left = 0;
        let mut result = 0;
        let mut total = 0;
        for right in 0..n {
            if right > 0 {
                total += (right - left) as i32 * (nums[right] - nums[right - 1]);
            }
            while total > k && left < right {
                total = total - nums[right] + nums[left];
                left += 1;
            }
            if total <= k {
                result = result.max(right - left + 1);
            }
            // println!(
            //     "left={}, right={}, total={}, result={}",
            //     left, right, total, result
            // );
        }
        result as i32
    }
}
