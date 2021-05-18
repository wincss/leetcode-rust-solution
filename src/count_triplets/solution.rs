use crate::*;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut sum = vec![0];
        let mut last = 0;
        for i in arr {
            last ^= i;
            sum.push(last);
        }
        let mut result = 0;
        for i in 0..n {
            for j in i + 1..n {
                for k in j..n {
                    if sum[i] ^ sum[j] == sum[k + 1] ^ sum[j] {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}
