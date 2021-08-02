use crate::*;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums2 = nums.clone();
        nums2.sort();
        let mut result = n as i32;
        for i in 0..n {
            if nums[i] == nums2[i] {
                result -= 1;
            } else {
                break;
            }
        }
        if result == 0 {
            return result;
        }
        for i in (0..n).rev() {
            if nums[i] == nums2[i] {
                result -= 1;
            } else {
                break;
            }
        }
        result
    }
}
