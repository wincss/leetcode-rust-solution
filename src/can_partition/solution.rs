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
        let n = nums.len();
        let mut dp = vec![vec![false; sum as usize + 1]; n + 1];
        dp[n][0] = true;
        for i in (0..n).rev() {
            for j in 0..=sum as usize {
                dp[i][j] = dp[i + 1][j] || (j as i32 >= nums[i] && dp[i + 1][j - nums[i] as usize])
            }
        }
        dp[0][sum as usize]
    }
}
