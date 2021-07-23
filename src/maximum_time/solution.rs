use crate::*;

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut time: Vec<char> = time.chars().collect();
        if time[0] == '?' {
            if time[1] == '?' || time[1] < '4' {
                time[0] = '2';
            } else {
                time[0] = '1';
            }
        }
        if time[1] == '?' {
            if time[0] == '2' {
                time[1] = '3';
            } else {
                time[1] = '9';
            }
        }
        if time[3] == '?' {
            time[3] = '5';
        }
        if time[4] == '?' {
            time[4] = '9';
        }
        time.into_iter().collect()
    }
}
