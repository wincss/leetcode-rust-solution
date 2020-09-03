use crate::*;

use std::collections::VecDeque;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board.get(0).map(|v| v.len()).unwrap_or(0);

        fn flood_fill(board: &mut Vec<Vec<char>>, n: usize, m: usize, x: usize, y: usize) {
            board[x][y] = 'K';
            let mut queue = VecDeque::new();
            queue.push_back((x, y));
            while let Some((x, y)) = queue.pop_front() {
                for &(dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                        // println!("nx={}={}, ny={}={}", nx, nx as usize, ny, ny as usize);

                        let nx = nx as usize;
                        let ny = ny as usize;
                        if board[nx][ny] == 'O' {
                            board[nx][ny] = 'K';
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
        for x in 0..n {
            if board[x][0] == 'O' {
                flood_fill(board, n, m, x, 0);
            }
            if board[x][m - 1] == 'O' {
                flood_fill(board, n, m, x, m - 1);
            }
        }
        for y in 0..m {
            if board[0][y] == 'O' {
                flood_fill(board, n, m, 0, y);
            }
            if board[n - 1][y] == 'O' {
                flood_fill(board, n, m, n - 1, y);
            }
        }
        for x in 0..n {
            for y in 0..m {
                if board[x][y] == 'O' {
                    board[x][y] = 'X';
                } else if board[x][y] == 'K' {
                    board[x][y] = 'O';
                }
            }
        }
    }
}
