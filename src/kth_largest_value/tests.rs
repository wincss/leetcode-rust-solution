use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1),
        7
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2),
        5
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3),
        4
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 4),
        0
    );
}
