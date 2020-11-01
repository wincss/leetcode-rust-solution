use crate::*;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();
        let mut result = vec![];
        let mut p2 = 0;
        let mut last = None;
        for i in nums1.into_iter() {
            if Some(i) == last {
                continue;
            }
            while p2 < nums2.len() && nums2[p2] < i {
                p2 += 1;
            }
            if p2 == nums2.len() {
                break;
            }
            if nums2[p2] == i {
                result.push(i);
            }
            last = Some(i);
        }
        result
    }
}
