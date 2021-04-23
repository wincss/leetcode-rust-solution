use crate::*;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n < 2 {
            return nums;
        }
        let mut nums = nums;
        nums.sort();
        let mut max_idx = 0;
        let mut dp = vec![];
        for i in 0..n {
            dp.push((1, 0));
            for j in (0..i).rev() {
                if nums[i] % nums[j] == 0 {
                    if dp[j].0 >= dp[i].0 {
                        dp[i] = (dp[j].0 + 1, j);
                    }
                }
            }
            if dp[i].0 > dp[max_idx].0 {
                max_idx = i;
            }
        }
        // println!("{:?}", dp);

        let mut result = vec![];
        loop {
            result.push(nums[max_idx]);
            if dp[max_idx].0 == 1 {
                break;
            }
            max_idx = dp[max_idx].1;
        }
        result
    }
}
