use crate::*;

#[test]
fn case_1() {
    let mut input = vec![1, 2, 3];
    Solution::next_permutation(&mut input);
    assert_eq!(input, vec![1, 3, 2]);
}

#[test]
fn case_2() {
    let mut input = vec![3, 2, 1];
    Solution::next_permutation(&mut input);
    assert_eq!(input, vec![1, 2, 3]);
}

#[test]
fn case_3() {
    let mut input = vec![1, 1, 5];
    Solution::next_permutation(&mut input);
    assert_eq!(input, vec![1, 5, 1]);
}
