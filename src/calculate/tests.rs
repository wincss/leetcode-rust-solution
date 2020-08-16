use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
}

#[test]
fn case_2() {
    assert_eq!(Solution::calculate("3/2".to_string()), 1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::calculate("3+5 / 2".to_string()), 5);
}

#[test]
fn case_4() {
    assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
}

#[test]
fn case_5() {
    assert_eq!(Solution::calculate("2-1 + 2".to_string()), 3);
}

#[test]
fn case_6() {
    assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
}

#[test]
fn case_7() {
    assert_eq!(Solution::calculate("(3+2)*(1+2)".to_string()), 15);
}
