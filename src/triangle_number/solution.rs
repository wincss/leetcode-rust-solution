use crate::*;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();
        let mut s = 0;
        for i in 0..n {
            for j in i + 1..n {
                let diff = nums[j] - nums[i];
                let sum = nums[i] + nums[j];
                let mut l1 = j + 1;
                let mut h = n;
                while l1 < h {
                    let mid = (l1 + h) / 2;
                    if diff < nums[mid] {
                        h = mid;
                    } else {
                        l1 = mid + 1;
                    }
                }
                let mut l2 = j + 1;
                h = n;
                while l2 < h {
                    let mid = (l2 + h) / 2;
                    if sum > nums[mid] {
                        l2 = mid + 1;
                    } else {
                        h = mid;
                    }
                }
                s += (l2 as i32 - l1 as i32).max(0);
            }
        }
        s
    }
}
