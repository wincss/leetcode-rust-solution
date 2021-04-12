use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_string());
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::largest_number(vec![3, 30, 34, 5, 9]),
        "9534330".to_string()
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::largest_number(vec![1]), "1".to_string());
}

#[test]
fn case_4() {
    assert_eq!(Solution::largest_number(vec![0, 0]), "0".to_string());
}
