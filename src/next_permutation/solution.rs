use crate::*;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut n = nums.len();
        let mut j = 0_usize;
        for i in (1..n).rev() {
            if nums[i - 1] < nums[i] {
                j = i;
                break;
            }
        }
        if j > 0 {
            for i in (j..n).rev() {
                if nums[j - 1] < nums[i] {
                    let t = nums[i];
                    nums[i] = nums[j - 1];
                    nums[j - 1] = t;
                    break;
                }
            }
        }
        n -= 1;
        while j < n {
            let t = nums[j];
            nums[j] = nums[n];
            nums[n] = t;
            n -= 1;
            j += 1;
        }
    }
}
