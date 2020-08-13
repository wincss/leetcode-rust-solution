use crate::*;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        fn helper(nums1: &Vec<i32>, nums2: &Vec<i32>, k: usize) -> i32 {
            let mut k = k;
            let mut p1 = 0;
            let mut p2 = 0;
            while k > 0 {
                let m1 = std::cmp::min(p1 + (k - 1) / 2, nums1.len() - 1);
                let m2 = std::cmp::min(p2 + (k - 1) / 2, nums2.len() - 1);
                if nums1[m1] > nums2[m2] {
                    k -= m2 - p2 + 1;
                    p2 = m2 + 1;
                } else {
                    k -= m1 - p1 + 1;
                    p1 = m1 + 1;
                }
            }
            std::cmp::min(nums1[p1], nums2[p2])
        }

        let l1 = nums1.len();
        let l2 = nums2.len();
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.push(std::i32::MAX);
        nums2.push(std::i32::MAX);
        let k = (l1 + l2) >> 1;
        let mut ans = helper(&nums1, &nums2, k) as f64;
        if (l1 + l2) & 1 == 0 {
            ans = (ans + helper(&nums1, &nums2, k - 1) as f64) / 2_f64;
        }
        ans
    }
}
