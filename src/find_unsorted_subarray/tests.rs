use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
}
