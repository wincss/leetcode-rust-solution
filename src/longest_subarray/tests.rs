use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::longest_subarray(vec![8, 2, 4, 7], 4), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0),
        3
    );
}

#[test]
fn case_4() {
    assert_eq!(Solution::longest_subarray(vec![10, 1, 2, 10], 1), 2);
}
