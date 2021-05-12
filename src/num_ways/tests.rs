use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_ways(3, 2), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_ways(2, 4), 2);
}

#[test]
fn case_3() {
    assert_eq!(Solution::num_ways(4, 2), 8);
}

#[test]
fn case_4() {
    assert_eq!(Solution::num_ways(27, 7), 127784505);
}
