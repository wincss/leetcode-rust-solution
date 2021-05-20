use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
        3
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
        2
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::max_uncrossed_lines(vec![1, 2, 2, 2, 1, 2], vec![1]),
        1
    );
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::max_uncrossed_lines(
            vec![4, 4, 4, 2, 4, 1, 4, 3, 2, 4, 4, 2, 2, 2, 5, 2, 5, 4, 5, 1],
            vec![2, 1, 5, 4, 2, 3, 5, 1, 2, 2]
        ),
        6
    );
}
