use crate::*;

#[test]
fn case_1_1() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn case_1_2() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn case_2_1() {
    assert_eq!(Solution::max_profit_ii(vec![7, 1, 5, 3, 6, 4]), 7);
}

#[test]
fn case_2_2() {
    assert_eq!(Solution::max_profit_ii(vec![1, 2, 3, 4, 5]), 4);
}

#[test]
fn case_2_3() {
    assert_eq!(Solution::max_profit_ii(vec![7, 6, 4, 3, 1]), 0);
}
