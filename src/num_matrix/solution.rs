#[allow(dead_code)]
pub struct NumMatrix {
    prefix_sum: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
impl NumMatrix {
    #[allow(dead_code)]
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        let m = if n > 0 { matrix[0].len() } else { 0 };
        let mut prefix_sum = vec![];
        prefix_sum.push(vec![0; m + 1]);
        for i in 0..n {
            let mut line_sum = 0;
            prefix_sum.push(vec![0]);
            for j in 0..m {
                line_sum += matrix[i][j];
                let last_sum = prefix_sum[i][j + 1] + line_sum;
                prefix_sum.last_mut().unwrap().push(last_sum);
            }
        }
        Self { prefix_sum }
    }
    #[allow(dead_code)]
    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        self.prefix_sum[row2 + 1][col2 + 1] + self.prefix_sum[row1][col1]
            - self.prefix_sum[row1][col2 + 1]
            - self.prefix_sum[row2 + 1][col1]
    }
}
