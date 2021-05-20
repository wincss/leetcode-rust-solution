use crate::*;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let mut dp = vec![0; l2];
        let mut topleft;
        for i in 0..l1 {
            topleft = dp[0];
            dp[0] = dp[0].max(if nums1[i] == nums2[0] { 1 } else { 0 });
            for j in 1..l2 {
                if nums1[i] == nums2[j] {
                    let tmp = dp[j];
                    dp[j] = topleft + 1;
                    topleft = tmp;
                } else {
                    topleft = dp[j];
                    dp[j] = dp[j].max(dp[j - 1]);
                }
            }
        }
        dp[l2 - 1]
    }
}
