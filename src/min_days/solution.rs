use crate::*;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if (bloom_day.len() as i32) < m * k {
            return -1;
        }
        let mut days = bloom_day.clone();
        days.sort();
        let mut l = 0;
        let mut r = days.len() - 1;
        while l < r {
            let mid = l + (r - l) / 2;
            let midday = days[mid];
            let mut t = 0;
            let mut last = 0;
            for &d in bloom_day.iter() {
                if d <= midday {
                    last += 1;
                    if last == k {
                        t += 1;
                        last = 0;
                    }
                } else {
                    last = 0;
                }
                if t >= m {
                    break;
                }
            }
            if t >= m {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        days[l]
    }
}
