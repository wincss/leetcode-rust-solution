use crate::*;

use std::collections::HashSet;
fn check_answer(n: i32, k: i32) {
    let mut expect_num = 1;
    for i in 1..=k {
        expect_num = expect_num * (n + 1 - i) / i;
    }
    let mut output = Solution::combine(n, k);
    let mut checked_output = HashSet::new();
    while let Some(mut pair) = output.pop() {
        pair.sort();
        assert!(checked_output.insert(pair));
    }
    assert_eq!(checked_output.len() as i32, expect_num);
}
#[test]
fn case_1() {
    check_answer(4, 2);
}
