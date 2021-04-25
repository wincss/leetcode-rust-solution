use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5),
        15
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
}

#[test]
fn case_3() {
    assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
}
