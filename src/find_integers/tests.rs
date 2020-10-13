use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_integers(1), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_integers(5), 5);
}

#[test]
fn case_3() {
    assert_eq!(Solution::find_integers(100), 34);
}

#[test]
fn case_4() {
    assert_eq!(Solution::find_integers(1000), 144);
}

#[test]
fn case_5() {
    assert_eq!(Solution::find_integers(100000000), 514229);
}
