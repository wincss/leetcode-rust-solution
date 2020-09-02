use crate::*;

#[test]
fn case_1() {
    assert!(Solution::stone_game(vec![5, 3, 4, 5]));
}
#[test]
fn case_2() {
    assert!(!Solution::stone_game(vec![1, 5, 2]));
}

#[test]
fn case_3() {
    assert!(Solution::stone_game(vec![1, 5, 233, 7]));
}
