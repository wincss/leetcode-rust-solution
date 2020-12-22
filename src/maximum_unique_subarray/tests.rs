use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]),
        8
    );
}
