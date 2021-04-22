use crate::*;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut colsum = vec![];
        for j in 0..m {
            let mut s = 0;
            let mut curr = vec![0];
            for i in 0..n {
                s += matrix[i][j];
                curr.push(s);
            }
            colsum.push(curr);
        }

        let mut result = std::i32::MIN;

        for top in 0..n {
            for bottom in top + 1..=n {
                let mut s = 0;
                let mut curr = vec![0];
                for j in 0..m {
                    s += colsum[j][bottom] - colsum[j][top];
                    let idx = curr.binary_search(&(s - k)).unwrap_or_else(|x| x);
                    if idx < curr.len() {
                        result = result.max(s - curr[idx]);
                    }
                    let idx = curr.binary_search(&s).unwrap_or_else(|x| x);
                    curr.insert(idx, s);
                }
            }
        }

        result
    }
}
