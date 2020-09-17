use crate::*;

use std::collections::HashSet;
#[test]
fn case_1() {
    let mut input = vec![1, 1, 2];
    let output = Solution::permute_unique(input.clone());
    input.sort();
    let output_len = output.len();
    let unique = output.into_iter().collect::<HashSet<Vec<i32>>>();
    assert_eq!(unique.len(), output_len);
    // TODO
    assert_eq!(output_len, 3);
}
