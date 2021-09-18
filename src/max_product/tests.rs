use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_product(s!("leetcodecom")), 9);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_product(s!("bb")), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_product(s!("accbcaxxcxx")), 25);
}
