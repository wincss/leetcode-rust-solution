use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
        5
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::max_turbulence_size(vec![1, 3, 2, 4, 0, 5]), 6);
}

#[test]
fn case_5() {
    assert_eq!(Solution::max_turbulence_size(vec![2, 2, 5, 3, 6, 1]), 5);
}

#[test]
fn case_6() {
    assert_eq!(Solution::max_turbulence_size(vec![9, 9]), 1);
}
