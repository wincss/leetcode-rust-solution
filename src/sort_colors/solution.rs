use crate::*;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut p1 = 0;
        let mut p2 = n;
        let mut i = 0;
        while i < p2 {
            // println!("{:?}", nums);
            if nums[i] == 0 {
                let tmp = nums[i];
                nums[i] = nums[p1];
                nums[p1] = tmp;
                i += 1;
                p1 += 1;
            } else if nums[i] == 2 {
                p2 -= 1;
                let tmp = nums[i];
                nums[i] = nums[p2];
                nums[p2] = tmp;
            } else {
                i += 1;
            }
        }
    }
}
