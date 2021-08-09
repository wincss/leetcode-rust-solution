use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]),
        vec![3, 1, 2, 1, 1, 0]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]),
        vec![4, 1, 1, 1, 0]
    );
}
