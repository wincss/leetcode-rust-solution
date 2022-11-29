use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_operations_1827(vec![1, 1, 1]), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_operations_1827(vec![1, 5, 2, 4, 1]), 14);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_operations_1827(vec![8]), 0);
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::min_operations_1713(vec![5, 1, 3], vec![9, 4, 2, 3, 4]),
        2
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::min_operations_1713(vec![6, 4, 8, 1, 3, 2], vec![4, 7, 6, 2, 3, 8, 6, 1]),
        3
    );
}

#[test]
fn case_1758_1() {
    assert_eq!(Solution::min_operations_1758(s!("0100")), 1);
}

#[test]
fn case_1758_2() {
    assert_eq!(Solution::min_operations_1758(s!("10")), 0);
}

#[test]
fn case_1758_3() {
    assert_eq!(Solution::min_operations_1758(s!("1111")), 2);
}
