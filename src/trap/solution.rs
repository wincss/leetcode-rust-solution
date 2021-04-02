use crate::*;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 0 {
            return 0;
        }
        let mut highest = 0;
        let mut highest_idx = 0;
        for i in 0..n {
            if height[i] > highest {
                highest = height[i];
                highest_idx = i;
            }
        }

        let mut amount = 0;
        let mut level = 0;
        for i in 0..highest_idx {
            if height[i] > level {
                level = height[i];
            } else {
                amount += level - height[i];
            }
        }

        let mut level = 0;
        for i in (highest_idx + 1..n).rev() {
            if height[i] > level {
                level = height[i];
            } else {
                amount += level - height[i];
            }
        }

        amount
    }
}
