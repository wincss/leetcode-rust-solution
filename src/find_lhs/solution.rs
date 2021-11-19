use crate::*;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut longest = 0;
        for (left, &v) in nums.iter().enumerate() {
            let right = nums.partition_point(|&x| x <= v + 1);
            if right > left && nums[right - 1] != v {
                longest = longest.max(right - left);
            }
        }
        longest as i32
    }
}
