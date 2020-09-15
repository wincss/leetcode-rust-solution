use crate::*;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn fill(
            board: &mut Vec<Vec<char>>,
            empty: &[(usize, usize)],
            state: &mut Vec<i32>,
        ) -> bool {
            if empty.len() == 0 {
                return true;
            }
            let (i, j) = empty[0];
            for k in 1_u8..=9 {
                let v = 1 << k;
                if state[i] & v == 0
                    && state[9 + j] & v == 0
                    && state[18 + i / 3 * 3 + j / 3] & v == 0
                {
                    board[i][j] = (k + 48) as char;
                    state[i] |= v;
                    state[9 + j] |= v;
                    state[18 + i / 3 * 3 + j / 3] |= v;
                    if fill(board, &empty[1..], state) {
                        return true;
                    }
                    state[i] ^= v;
                    state[9 + j] ^= v;
                    state[18 + i / 3 * 3 + j / 3] ^= v;
                }
            }
            false
        }
        let mut state = vec![0; 27];
        let mut empty = vec![];
        for i in 0_usize..9 {
            for j in 0_usize..9 {
                if board[i][j] == '.' {
                    empty.push((i, j));
                } else {
                    let n = board[i][j] as u8 - 48;
                    state[i] |= 1 << n;
                    state[9 + j] |= 1 << n;
                    state[18 + i / 3 * 3 + j / 3] |= 1 << n;
                }
            }
        }
        fill(board, &empty[..], &mut state);
    }
}
