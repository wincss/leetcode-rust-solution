use crate::*;

use std::collections::VecDeque;

impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let diff = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        let mut board = board;
        let n = board.len();
        let m = board[0].len();
        let x = click[0] as usize;
        let y = click[1] as usize;
        if board[x][y] == 'M' {
            board[x][y] = 'X';
            return board;
        }
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        while let Some((x, y)) = queue.pop_front() {
            if board[x][y] != 'E' {
                continue;
            }
            let mines = diff
                .iter()
                .map(|&(dx, dy)| (x as i32 + dx, y as i32 + dy))
                .filter(|&(x, y)| (x >= 0 && x < n as i32 && y >= 0 && y < m as i32))
                .filter(|&(x, y)| board[x as usize][y as usize] == 'M')
                .count();
            if mines > 0 {
                board[x][y] = (mines + 48) as u8 as char;
            } else {
                board[x][y] = 'B';
                queue.extend(
                    diff.iter()
                        .map(|&(dx, dy)| (x as i32 + dx, y as i32 + dy))
                        .filter(|&(x, y)| (x >= 0 && x < n as i32 && y >= 0 && y < m as i32))
                        .filter(|&(x, y)| board[x as usize][y as usize] == 'E')
                        .map(|(x, y)| (x as usize, y as usize)),
                );
            }
        }
        board
    }
}
