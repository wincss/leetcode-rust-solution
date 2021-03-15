use crate::*;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        enum Direction {
            Left,
            Right,
            Up,
            Down,
        }
        let mut bottom = matrix.len();
        let mut right = if bottom > 0 { matrix[0].len() } else { 0 };
        let mut top = 0;
        let mut left = 0;
        let n = bottom * right;

        let mut result = vec![];
        let mut direct = Direction::Right;

        let mut i = 0;
        let mut j = 0;
        for _ in 0..n {
            result.push(matrix[i][j]);
            match direct {
                Direction::Left => {
                    if j > left {
                        j -= 1;
                    } else {
                        i -= 1;
                        bottom -= 1;
                        direct = Direction::Up;
                    }
                }
                Direction::Right => {
                    if j < right - 1 {
                        j += 1;
                    } else {
                        i += 1;
                        top += 1;
                        direct = Direction::Down;
                    }
                }
                Direction::Up => {
                    if i > top {
                        i -= 1;
                    } else {
                        j += 1;
                        left += 1;
                        direct = Direction::Right;
                    }
                }
                Direction::Down => {
                    if i < bottom - 1 {
                        i += 1;
                    } else {
                        j -= 1;
                        right -= 1;
                        direct = Direction::Left;
                    }
                }
            }
        }
        result
    }
}
