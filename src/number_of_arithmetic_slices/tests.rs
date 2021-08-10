use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::number_of_arithmetic_slices_413(vec![1, 2, 3, 4]),
        3
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::number_of_arithmetic_slices_413(vec![1]), 0);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::number_of_arithmetic_slices_413(vec![1, 2, 3, 5, 7]),
        2
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::number_of_arithmetic_slices_446(vec![2, 4, 6, 8, 10]),
        7
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::number_of_arithmetic_slices_446(vec![7, 7, 7, 7, 7]),
        16
    );
}
#[test]
fn case_6() {
    assert_eq!(
        Solution::number_of_arithmetic_slices_446(vec![0, 2000000000, -294967296]),
        0
    );
}
