use crate::*;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        if n == 0 {
            return;
        }
        let mut old_idx = 0;
        let mut new_idx = 0;
        for (i, &v) in arr.iter().enumerate() {
            old_idx = i;
            new_idx += if v == 0 { 2 } else { 1 };
            if new_idx >= n {
                break;
            }
        }
        for i in (0..=old_idx).rev() {
            new_idx -= 1;
            if arr[i] == 0 {
                if new_idx < n {
                    arr[new_idx] = 0;
                }
                new_idx -= 1;
            }
            arr[new_idx] = arr[i];
        }
    }
}
