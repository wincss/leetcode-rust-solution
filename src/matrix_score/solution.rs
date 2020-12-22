use crate::*;

impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let mut a = a;
        let n = a.len();
        if n == 0 {
            return 0;
        }
        let m = a[0].len();
        for i in 0..n {
            if a[i][0] == 0 {
                for j in 0..m {
                    a[i][j] = 1 - a[i][j];
                }
            }
        }
        // println!("{:?}", a);
        for j in 1..m {
            let mut ones = 0;
            for i in 0..n {
                ones += a[i][j];
            }
            if ones < n as i32 - ones {
                for i in 0..n {
                    a[i][j] = 1 - a[i][j];
                }
            }
            // println!("{:?}", a);
        }

        let mut sum = 0;
        for i in 0..n {
            let mut num = 0;
            for j in 0..m {
                num = (num << 1) | a[i][j]
            }
            sum += num;
        }
        sum
    }
}
