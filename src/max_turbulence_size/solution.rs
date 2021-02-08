use crate::*;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n < 2 {
            return n as i32;
        }
        let mut direction = None;
        let mut left = 0;
        let mut result = 0;
        for i in 1..n {
            let key = (arr[i] - arr[i - 1]).signum();
            if direction.unwrap_or(-key) * key >= 0 {
                result = std::cmp::max(result, i - left);
                left = if key == 0 { i } else { i - 1 };
            }
            direction = Some(key);
            // println!(
            //     "arr[{}]={} last={} left={} key={} dir={:?}",
            //     i, arr[i], arr[i - 1], left, key, direction
            // )
        }

        std::cmp::max(result, n - left) as i32
    }
}
