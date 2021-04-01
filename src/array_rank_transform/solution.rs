use crate::*;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();

        let mut arr: Vec<_> = arr.into_iter().enumerate().map(|(x, y)| (y, x)).collect();
        arr.sort();
        let mut result = vec![0; n];
        let mut rank = 0;
        let mut last = None;
        for (val, idx) in arr {
            if !last.map(|v| v == val).unwrap_or(false) {
                rank += 1;
                last = Some(val);
            }
            result[idx] = rank;
        }
        result
    }
}
