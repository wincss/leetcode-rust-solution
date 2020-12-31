use crate::*;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let n = intervals.len();
        if n == 0 {
            return 0;
        }
        intervals.sort_by_key(|v| v[1]);
        let mut take = 1;
        let mut right = intervals[0][1];
        for i in 1..n {
            if intervals[i][0] >= right {
                take += 1;
                right = intervals[i][1];
            }
        }
        n as i32 - take
    }
}
