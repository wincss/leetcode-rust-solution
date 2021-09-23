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
        let mut trip = vec![0];

        let mut last_weight = 0;
        let mut last_port = 0;
        let mut last_move = 0;
        for item in boxes.iter() {
            last_weight += item[1];
            weight.push(last_weight);
            if item[0] != last_port {
                last_move += 1;
                last_port = item[0];
            }
            trip.push(last_move);
        }

        let mut dp = vec![0; n + 1];
        let mut left = 0;
        let mut left_same = 0;
        for right in 1..=n {
            while right - left > max_boxes || weight[right] - weight[left] > max_weight {
                left += 1;
            }
            dp[right] = dp[left] + trip[right] - trip[left] + 2;
            if trip[right] == trip[left] || trip[left + 1] == trip[left] {
                continue;
            }
            if left_same <= left {
                left_same = left + 1;
                while trip[left_same + 1] == trip[left_same] {
                    left_same += 1;
                }
            }
            dp[right] = dp[right].min(dp[left_same] + trip[right] - trip[left_same] + 2);
        }
        dp[n]
    }
}
