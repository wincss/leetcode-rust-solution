use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut x_min = std::i32::MAX;
        let mut x_max = std::i32::MIN;
        let mut y_min = std::i32::MAX;
        let mut y_max = std::i32::MIN;
        let mut area = 0;
        let mut points = HashMap::new();
        for rect in rectangles {
            x_min = x_min.min(rect[0]);
            x_max = x_max.max(rect[2]);
            y_min = y_min.min(rect[1]);
            y_max = y_max.max(rect[3]);
            area += (rect[3] - rect[1]) * (rect[2] - rect[0]);
            *points.entry((rect[0], rect[1])).or_insert(0) += 1;
            *points.entry((rect[0], rect[3])).or_insert(0) += 1;
            *points.entry((rect[2], rect[1])).or_insert(0) += 1;
            *points.entry((rect[2], rect[3])).or_insert(0) += 1;
        }
        (x_max - x_min) * (y_max - y_min) == area
            && points.into_iter().all(|(point, num)| {
                if point == (x_min, y_min)
                    || point == (x_min, y_max)
                    || point == (x_max, y_min)
                    || point == (x_max, y_max)
                {
                    num == 1
                } else {
                    num == 2 || num == 4
                }
            })
    }
}
