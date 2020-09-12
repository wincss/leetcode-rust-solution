use crate::*;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn go(
            board: &mut Vec<Vec<char>>,
            n: usize,
            m: usize,
            i: usize,
            j: usize,
            word: &[char],
        ) -> bool {
            if word.len() == 0 {
                return true;
            }
            if word[0] == board[i][j] {
                if word.len() == 1 {
                    return true;
                }
                board[i][j] = '.';
                if i > 0 && go(board, n, m, i - 1, j, &word[1..]) {
                    return true;
                }
                if i < n - 1 && go(board, n, m, i + 1, j, &word[1..]) {
                    return true;
                }
                if j > 0 && go(board, n, m, i, j - 1, &word[1..]) {
                    return true;
                }
                if j < m - 1 && go(board, n, m, i, j + 1, &word[1..]) {
                    return true;
                }
                board[i][j] = word[0];
            }
            false
        }
        let word: Vec<char> = word.chars().collect();
        let mut board = board;
        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            for j in 0..m {
                if word[0] == board[i][j] {
                    if go(&mut board, n, m, i, j, &word[..]) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
