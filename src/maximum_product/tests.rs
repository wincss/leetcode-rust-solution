use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
}

#[test]
fn case_2() {
    assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
}

#[test]
fn case_3() {
    assert_eq!(Solution::maximum_product(vec![-1, 2, 3]), -6);
}

#[test]
fn case_4() {
    assert_eq!(Solution::maximum_product(vec![-2, -1, 9, 10]), 20);
}
