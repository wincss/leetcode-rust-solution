use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
}

#[test]
fn case_2() {
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}
