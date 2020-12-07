use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::minimum_incompatibility(vec![1, 2, 1, 4], 2), 4);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::minimum_incompatibility(vec![6, 3, 8, 1, 3, 1, 2, 2], 4),
        6
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::minimum_incompatibility(vec![5, 3, 3, 6, 3, 3], 3),
        -1
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::minimum_incompatibility(
            vec![5, 3, 4, 14, 5, 11, 10, 11, 14, 2, 10, 16, 9, 12, 2, 3],
            16
        ),
        0
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::minimum_incompatibility(
            vec![6, 8, 5, 16, 8, 12, 11, 7, 13, 16, 15, 14, 7, 9, 1, 10],
            4
        ),
        19
    );
}

#[test]
fn case_6() {
    assert_eq!(
        Solution::minimum_incompatibility(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            2
        ),
        14
    );
}
