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

#[test]
fn case_3_1() {
    assert_eq!(Solution::max_profit_with_fee(vec![1, 3, 2, 8, 4, 9], 2), 8);
}

#[test]
fn case_4_1() {
    assert_eq!(Solution::max_profit_iv(2, vec![2, 4, 1]), 2);
}

#[test]
fn case_4_2() {
    assert_eq!(Solution::max_profit_iv(2, vec![3, 2, 6, 5, 0, 3]), 7);
}

#[test]
fn case_4_3() {
    assert_eq!(Solution::max_profit_iv(2, vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
}
