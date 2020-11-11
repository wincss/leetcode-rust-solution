use crate::*;

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; a.len()];
        let mut even_pos = 0_usize;
        let mut odd_pos = 1_usize;
        for i in a.into_iter() {
            if i & 1 == 1 {
                result[odd_pos] = i;
                odd_pos += 2;
            } else {
                result[even_pos] = i;
                even_pos += 2;
            }
        }
        result
    }
}
