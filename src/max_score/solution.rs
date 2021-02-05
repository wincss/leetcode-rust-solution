use crate::*;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let r = card_points.len() - k as usize;
        let mut result = std::i32::MAX;
        let mut sum = 0;
        let mut total = 0;
        for (i, &v) in card_points.iter().enumerate() {
            total += v;
            sum += v;
            if i >= r {
                sum -= card_points[i - r];
            }
            if i + 1 >= r {
                result = std::cmp::min(result, sum);
            }
        }
        total - result
    }
}
