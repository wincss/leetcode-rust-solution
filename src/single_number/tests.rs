use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::single_number(vec![-2, -2, 1, 1, 4, 1, 4, 4, -4, -2]),
        -4
    );
}
