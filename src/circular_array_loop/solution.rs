use crate::*;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let n = nums.len();
        fn next(x: usize, nums: &Vec<i32>) -> usize {
            ((x as i32 + nums[x]) % (nums.len() as i32) + nums.len() as i32) as usize % nums.len()
        }

        for i in 0..n {
            if nums[i] == 0 {
                continue;
            }
            let mut slow = i;
            let mut fast = next(i, &nums);

            while nums[slow] * nums[fast] > 0 && nums[slow] * nums[next(fast, &nums)] > 0 {
                if slow == fast {
                    if slow == next(slow, &nums) {
                        break;
                    }
                    return true;
                }
                slow = next(slow, &nums);
                fast = next(next(fast, &nums), &nums);
            }

            let mut slow = i;
            while nums[slow] * nums[next(slow, &nums)] > 0 {
                let tmp = slow;
                slow = next(slow, &nums);
                nums[tmp] = 0;
            }
        }
        false
    }
}
