use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::arrange_coins(1), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::arrange_coins(5), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::arrange_coins(8), 3);
}
