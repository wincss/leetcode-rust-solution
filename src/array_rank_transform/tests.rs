use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::array_rank_transform(vec![40, 10, 20, 30]),
        vec![4, 1, 2, 3]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::array_rank_transform(vec![100, 100, 100]),
        vec![1, 1, 1]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
        vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
    );
}
