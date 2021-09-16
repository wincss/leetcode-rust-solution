use crate::*;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = 9;
        for i in 0..n {
            let mut row_check = vec![false; n];
            let mut col_check = vec![false; n];
            let mut box_check = vec![false; n];
            for j in 0..n {
                let v = board[i][j] as u8 as usize;
                if v > 48 {
                    if row_check[v - 49] {
                        return false;
                    }
                    row_check[v - 49] = true;
                }
                let v = board[j][i] as u8 as usize;
                if v > 48 {
                    if col_check[v - 49] {
                        return false;
                    }
                    col_check[v - 49] = true;
                }
                let v = board[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3] as u8 as usize;
                if v > 48 {
                    if box_check[v - 49] {
                        return false;
                    }
                    box_check[v - 49] = true;
                }
            }
        }
        true
    }
}
