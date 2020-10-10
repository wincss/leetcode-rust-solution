use crate::*;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for &i in nums.iter() {
            sum += i;
        }
        if sum & 1 == 1 {
            return false;
        }
        sum >>= 1;
        let sum = sum as usize;
        let n = nums.len();
        let mut dp = vec![false; sum + 1];
        dp[sum] = true;
        for i in 0..n {
            for j in 0..=sum {
                if j >= nums[i] as usize {
                    dp[j - nums[i] as usize] |= dp[j]
                }
            }
            if dp[0] {
                break;
            }
        }
        dp[0]
    }
}
