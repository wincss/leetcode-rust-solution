use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::min_days(vec![1000000000, 1000000000], 1, 1),
        1000000000
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::min_days(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2),
        9
    );
}
