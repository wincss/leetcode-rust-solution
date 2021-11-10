use crate::*;
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut total = 0;
        let mut poisoned_end = 0;
        for i in time_series {
            if i < poisoned_end {
                total -= poisoned_end - i;
            }
            poisoned_end = i + duration;
            total += duration;
        }
        total
    }
}
