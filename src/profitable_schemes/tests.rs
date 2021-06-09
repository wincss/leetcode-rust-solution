use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::profitable_schemes(5, 3, vec![2, 2], vec![2, 3]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8]),
        7
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::profitable_schemes(10, 10, vec![9], vec![6]), 0);
}
