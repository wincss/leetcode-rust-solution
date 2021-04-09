use crate::*;

use std::collections::HashSet;
impl Solution {
    pub fn escape_maze(maze: Vec<Vec<String>>) -> bool {
        let maze: Vec<Vec<Vec<char>>> = maze
            .into_iter()
            .map(|v| v.into_iter().map(|v| v.chars().collect()).collect())
            .collect();
        fn go(
            maze: &Vec<Vec<Vec<char>>>,
            x: usize,
            y: usize,
            t: usize,
            m1: bool,
            m2: bool,
            n: usize,
            m: usize,
            maxt: usize,
            saved: &mut HashSet<(usize, usize, usize, bool, bool)>,
        ) -> bool {
            if x == n - 1 && y == m - 1 {
                return true;
            }
            if t + 1 == maxt {
                return false;
            }
            let key = (x, y, t, m1, m2);
            if saved.contains(&key) {
                return false;
            }

            for &(dx, dy) in [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if maze[t + 1][nx][ny] == '.' {
                        if go(maze, nx, ny, t + 1, m1, m2, n, m, maxt, saved) {
                            return true;
                        }
                    } else {
                        if !m1 {
                            if go(maze, nx, ny, t + 1, true, m2, n, m, maxt, saved) {
                                return true;
                            }
                        }
                        if !m2 {
                            for nt in t + 1..maxt {
                                if go(maze, nx, ny, nt, m1, true, n, m, maxt, saved) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            saved.insert(key);
            false
        }
        let maxt = maze.len();
        let n = maze[0].len();
        let m = maze[0][0].len();
        let mut saved = HashSet::new();
        go(&maze, 0, 0, 0, false, false, n, m, maxt, &mut saved)
    }
}
