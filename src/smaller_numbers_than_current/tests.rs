use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![5, 5, 5]),
        vec![0, 0, 0]
    );
}
