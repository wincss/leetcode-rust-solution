use crate::*;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = if n > 0 { grid[0].len() } else { 0 };
        let mut result = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    if i > 0 && grid[i - 1][j] == 0 || i == 0 {
                        result += 1;
                    }
                    if i < n - 1 && grid[i + 1][j] == 0 || i == n - 1 {
                        result += 1;
                    }
                    if j > 0 && grid[i][j - 1] == 0 || j == 0 {
                        result += 1;
                    }
                    if j < m - 1 && grid[i][j + 1] == 0 || j == m - 1 {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}
