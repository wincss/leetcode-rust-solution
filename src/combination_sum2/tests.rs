use crate::*;

use std::collections::HashSet;

fn check_answer(candidates: Vec<i32>, target: i32, expect: Vec<Vec<i32>>) {
    let output = Solution::combination_sum2(candidates, target);
    let length = output.len();
    let output: HashSet<Vec<i32>> = output
        .into_iter()
        .map(|mut v| {
            v.sort();
            v
        })
        .collect();
    assert_eq!(length, output.len());
    let expect: HashSet<Vec<i32>> = expect.into_iter().collect();
    assert_eq!(output, expect);
}
#[test]
fn case_1() {
    check_answer(
        vec![10, 1, 2, 7, 6, 1, 5],
        8,
        vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]],
    );
}

#[test]
fn case_2() {
    check_answer(vec![2, 5, 2, 1, 2], 5, vec![vec![1, 2, 2], vec![5]]);
}
