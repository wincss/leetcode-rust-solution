use crate::*;

use std::collections::HashSet;

fn check_answer(k: i32, n: i32, expect: Vec<Vec<i32>>) {
    let output = Solution::combination_sum3(k, n);
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
        3,
        7,
        vec![vec![1, 2,4]],
    );
}

#[test]
fn case_2() {
    check_answer(
        3,
        9,
        vec![vec![1,2,6], vec![1,3,5], vec![2,3,4]],
    );
}