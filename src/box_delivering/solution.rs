use crate::*;

impl Solution {
    pub fn box_delivering(
        boxes: Vec<Vec<i32>>,
        _ports_count: i32,
        max_boxes: i32,
        max_weight: i32,
    ) -> i32 {
        let n = boxes.len();
        let max_boxes = max_boxes as usize;

        let mut weight = vec![0];
        let mut trip = vec![];

        let mut last_weight = 0;
        let mut last_port = 0;
        let mut last_move = 0;
        for item in boxes.iter() {
            last_weight += item[1] as i64;
            weight.push(last_weight);
            if item[0] != last_port {
                last_move += 1;
                last_port = item[0];
            }
            trip.push(last_move);
        }
        let mut dp = vec![0; n + 1];
        let mut left = 0;
        for right in 1..=n {
            while right - left > max_boxes || weight[right] - weight[left] > max_weight as i64 {
                left += 1;
            }
            dp[right] = dp[left] + trip[right - 1] - trip[left] + 2;
            if trip[right - 1] == trip[left] || (left > 0 && boxes[left][0] != boxes[left - 1][0]) {
                continue;
            }
            let mut left_same = left + 1;
            while left_same < right - 1 && boxes[left_same][0] == boxes[left_same - 1][0] {
                left_same += 1;
            }
            dp[right] = dp[right].min(dp[left_same] + trip[right - 1] - trip[left_same] + 2);
        }
        dp[n]
    }
}
