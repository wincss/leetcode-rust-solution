use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
}
#[test]
fn case_2() {
    assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
}
#[test]
fn case_3() {
    assert_eq!(Solution::find_paths(3, 8, 0, 2, 0), 0);
}
