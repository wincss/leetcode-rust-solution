use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]),
        2
    );
}
