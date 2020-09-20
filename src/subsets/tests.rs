use crate::*;

use std::collections::HashSet;
#[test]
fn case_1() {
    let output = Solution::subsets(vec![1, 2, 3]);
    let expect = vec![
        vec![3],
        vec![1],
        vec![2],
        vec![1, 2, 3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2],
        vec![],
    ]
    .into_iter()
    .map(|mut v| {
        v.sort();
        v
    })
    .collect::<HashSet<Vec<i32>>>();
    let output_set = output
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect::<HashSet<Vec<i32>>>();
    assert_eq!(output_set, expect);
}
