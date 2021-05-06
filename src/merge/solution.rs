use crate::*;

impl Solution {
    pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();
        let mut output: Vec<Vec<i32>> = vec![];
        for interval in intervals.into_iter() {
            if let Some(last) = output.last_mut() {
                if interval[0] > last[1] {
                    output.push(interval);
                } else {
                    last[1] = last[1].max(interval[1]);
                }
            } else {
                output.push(interval);
            }
        }
        output
    }

    pub fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut idx = nums1.len();
        while idx > 0 {
            idx -= 1;
            if m > 0 && n > 0 {
                if nums1[m - 1] > nums2[n - 1] {
                    m -= 1;
                    nums1[idx] = nums1[m];
                } else {
                    n -= 1;
                    nums1[idx] = nums2[n];
                }
            } else if m > 0 {
                m -= 1;
                nums1[idx] = nums1[m];
            } else {
                n -= 1;
                nums1[idx] = nums2[n];
            }
        }
    }
}
