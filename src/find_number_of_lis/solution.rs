use crate::*;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![];
        let mut count = vec![];
        let mut max_length = 0;
        let mut result = 0;
        for i in 0..n {
            dp.push(1);
            count.push(1);
            for j in 0..i {
                if nums[i] > nums[j] {
                    if dp[j] + 1 > dp[i] {
                        dp[i] = dp[j] + 1;
                        count[i] = count[j];
                    } else if dp[j] + 1 == dp[i] {
                        count[i] += count[j];
                    }
                }
            }
            if dp[i] > max_length {
                max_length = dp[i];
                result = count[i];
            } else if dp[i] == max_length {
                result += count[i];
            }
            // println!("{:?} {:?} {} {}", dp, count, max_length, result);
        }
        result
    }
}
