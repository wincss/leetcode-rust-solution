use crate::*;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut subsets = vec![];
        for key in 0..(1 << n) {
            subsets.push(
                nums.iter()
                    .enumerate()
                    .filter_map(|(i, &v)| if key & (1 << i) > 0 { Some(v) } else { None })
                    .collect(),
            );
        }
        subsets
    }
}
