use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
}

#[test]
fn case_2() {
    assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
}

#[test]
fn case_3() {
    assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
}

#[test]
fn case_4() {
    assert_eq!(Solution::max_score(vec![1, 1000, 1], 1), 1);
}

#[test]
fn case_5() {
    assert_eq!(
        Solution::max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3),
        202
    );
}
