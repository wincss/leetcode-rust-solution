use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::min_steps(3), 3);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_steps(1), 0);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_steps(1000), 21);
}

#[test]
fn case_4() {
    assert_eq!(Solution::min_steps(935), 33);
}
