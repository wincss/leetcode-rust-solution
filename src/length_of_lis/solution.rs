use crate::*;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut d = vec![];
        for i in nums.into_iter() {
            let idx = d.binary_search(&i).unwrap_or_else(|x| x);
            if idx < d.len() {
                d[idx] = i;
            } else {
                d.push(i);
            }
        }
        d.len() as i32
    }
}
