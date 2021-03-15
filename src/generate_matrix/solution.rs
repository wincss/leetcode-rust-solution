use crate::*;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        enum Direction {
            Left,
            Right,
            Up,
            Down,
        }

        let n = n as usize;
        let mut i = 0;
        let mut j = 0;
        let mut direction = Direction::Right;
        let mut result = vec![vec![0; n]; n];
        for k in 1..=n * n {
            result[i][j] = k as i32;
            match direction {
                Direction::Right => {
                    if j + 1 > n - 1 || result[i][j + 1] > 0 {
                        direction = Direction::Down;
                        i += 1;
                    } else {
                        j += 1;
                    }
                }
                Direction::Down => {
                    if i + 1 > n - 1 || result[i + 1][j] > 0 {
                        direction = Direction::Left;
                        j -= 1;
                    } else {
                        i += 1;
                    }
                }
                Direction::Left => {
                    if j == 0 || result[i][j - 1] > 0 {
                        direction = Direction::Up;
                        i -= 1;
                    } else {
                        j -= 1;
                    }
                }
                Direction::Up => {
                    if i == 0 || result[i - 1][j] > 0 {
                        direction = Direction::Right;
                        j += 1;
                    } else {
                        i -= 1;
                    }
                }
            }
        }
        result
    }
}
