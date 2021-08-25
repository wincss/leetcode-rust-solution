use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
}

#[test]
fn case_2() {
    assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
}

#[test]
fn case_3() {
    assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
}
