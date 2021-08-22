use crate::*;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let n = n as usize;
        if n == 0 {
            return 0;
        }
        let mut nums = vec![0; n + 1];
        let mut mx = 1;
        nums[1] = 1;
        for i in 2..=n {
            if i % 2 == 0 {
                nums[i] = nums[i / 2];
            } else {
                nums[i] = nums[i - 1] + nums[i / 2 + 1];
            }
            mx = mx.max(nums[i]);
        }
        mx
    }
}
