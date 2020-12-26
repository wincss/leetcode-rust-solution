use crate::*;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix[0].len();
        if m == 0 {
            return 0;
        }
        let mut left = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == '1' {
                    left[i][j] = if j > 0 { left[i][j - 1] + 1 } else { 1 };
                }
            }
        }
        // println!("m={},n={},left={:?}", m, n, left);
        let mut result = 0;
        for j in 0..m {
            let mut up = vec![0; n];
            let mut st: Vec<usize> = vec![];
            for i in 0..n {
                while let Some(&v) = st.last() {
                    if left[v][j] >= left[i][j] {
                        st.pop();
                    } else {
                        break;
                    }
                }
                up[i] = st.last().map(|&v| v as i32).unwrap_or(-1);
                st.push(i);
            }
            let mut down = vec![0; n];
            let mut st: Vec<usize> = vec![];
            for i in (0..n).rev() {
                while let Some(&v) = st.last() {
                    if left[v][j] >= left[i][j] {
                        st.pop();
                    } else {
                        break;
                    }
                }
                down[i] = st.last().map(|&v| v as i32).unwrap_or(n as i32);
                st.push(i);
            }
            for i in 0..n {
                // println!("j={},i={},down={:?},up={:?}", j, i, down, up);
                result = std::cmp::max(result, (down[i] - up[i] - 1) * left[i][j]);
            }
        }
        result
    }
}
