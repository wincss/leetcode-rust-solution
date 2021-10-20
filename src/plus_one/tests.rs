use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::plus_one(vv![1, 2, 3]), vv![1, 2, 4]);
}

#[test]
fn case_2() {
    assert_eq!(Solution::plus_one(vv![4, 3, 2, 1]), vv![4, 3, 2, 2]);
}

#[test]
fn case_3() {
    assert_eq!(Solution::plus_one(vv![0]), vv![1]);
}

#[test]
fn case_4() {
    assert_eq!(Solution::plus_one(vv![9]), vv![1, 0]);
}
