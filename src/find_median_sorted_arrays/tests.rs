use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2_f64
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.5
    )
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![3, 4]), 3.5)
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 4]),
        2.5
    )
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4, 5, 6]),
        3.5
    )
}
