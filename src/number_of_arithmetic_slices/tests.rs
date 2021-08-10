use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 5, 7]),
        2
    );
}
