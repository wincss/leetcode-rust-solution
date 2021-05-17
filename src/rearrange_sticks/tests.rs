use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::rearrange_sticks(3, 2), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::rearrange_sticks(5, 5), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::rearrange_sticks(20, 11), 647427950);
}
