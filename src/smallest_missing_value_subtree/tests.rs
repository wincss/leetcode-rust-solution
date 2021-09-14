use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::smallest_missing_value_subtree(vv![-1, 0, 0, 2], vv![1, 2, 3, 4]),
        vv![5, 1, 1, 1]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::smallest_missing_value_subtree(vv![-1, 0, 1, 0, 3, 3], vv![5, 4, 6, 2, 1, 3]),
        vv![7, 1, 1, 4, 2, 1]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::smallest_missing_value_subtree(
            vv![-1, 2, 3, 0, 2, 4, 1],
            vv![2, 3, 4, 5, 6, 7, 8]
        ),
        vv![1, 1, 1, 1, 1, 1, 1]
    );
}
