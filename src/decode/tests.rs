use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::decode_1720(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::decode_1720(vec![6, 2, 7, 3], 4),
        vec![4, 2, 0, 7, 4]
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::decode_1734(vec![3, 1]), vec![1, 2, 3]);
}

#[test]
fn case_4() {
    assert_eq!(Solution::decode_1734(vec![6, 5, 4, 6]), vec![2, 4, 1, 5, 3]);
}
