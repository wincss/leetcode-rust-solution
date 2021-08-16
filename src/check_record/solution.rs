use crate::*;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent = 0;
        let mut late = 0;
        let mut continous_late = false;
        for c in s.chars() {
            if c == 'A' {
                absent += 1;
            }
            if c == 'L' {
                late += 1;
                if late == 3 {
                    continous_late = true;
                }
            } else {
                late = 0;
            }
        }
        absent < 2 && !continous_late
    }
}
