use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_count(3, 3, vv![[2, 2], [3, 3]]), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_count(3, 3, vv![]), 9);
}
