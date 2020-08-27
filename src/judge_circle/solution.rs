use crate::*;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        for &i in moves.as_bytes() {
            match i {
                b'U' => x -= 1,
                b'D' => x += 1,
                b'L' => y -= 1,
                b'R' => y += 1,
                _ => unreachable!(),
            }
        }
        x == 0 && y == 0
    }
}
