use crate::*;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        let neighborhood = |i: usize| {
            (
                if i > 0 { nums[i - 1] } else { std::i32::MIN },
                nums[i],
                if i < n - 1 {
                    nums[i + 1]
                } else {
                    std::i32::MIN
                },
            )
        };
        let mut l = 0;
        let mut r = n;
        while l < r {
            let mid = l + (r - l) / 2;
            let (n1, n2, n3) = neighborhood(mid);
            if n1 < n2 && n2 > n3 {
                return mid as i32;
            } else if n1 < n2 && n2 < n3 {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}
