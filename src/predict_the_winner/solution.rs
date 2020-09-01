use crate::*;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        fn can_win(nums: &[i32], a: i32, b: i32, win_when_equal: bool) -> bool {
            let n = nums.len();
            if n == 0 {
                return a > b || (a == b && win_when_equal);
            }
            let ret = !(can_win(&nums[1..], b, a + nums[0], !win_when_equal)
                && can_win(&nums[..n - 1], b, a + nums[n - 1], !win_when_equal));
            // println!("{:?}, {}, {}, {}", nums, a, b, ret);
            return ret;
        }
        can_win(&nums[..], 0, 0, true)
    }
}
