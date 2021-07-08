use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::majority_element(vec![1, 2, 5, 9, 5, 9, 5, 5, 5]),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::majority_element(vec![3, 3, 4]), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::majority_element(vec![3, 2]), -1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::majority_element(vec![1]), 1);
}
