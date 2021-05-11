use crate::*;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = vec![0];
        let mut last = 0;
        for i in arr.into_iter() {
            last ^= i;
            sum.push(last);
        }
        let mut result = vec![];
        for item in queries.into_iter() {
            result.push(sum[item[1] as usize + 1] ^ sum[item[0] as usize]);
        }
        result
    }
}
