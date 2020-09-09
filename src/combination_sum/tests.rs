use crate::*;

use std::collections::HashSet;
#[test]
fn case_1() {
    let output: HashSet<Vec<i32>> = Solution::combination_sum(vec![2, 3, 6, 7], 7)
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    let expect: HashSet<Vec<i32>> = vec![vec![7], vec![2, 2, 3]].into_iter().collect();
    assert_eq!(output, expect);
}

#[test]
fn case_2() {
    let output: HashSet<Vec<i32>> = Solution::combination_sum(vec![2, 3, 5], 8)
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    let expect: HashSet<Vec<i32>> = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        .into_iter()
        .collect();
    assert_eq!(output, expect);
}
