use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_poisoned_duration(vv![1, 4], 2), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_poisoned_duration(vv![1, 2], 2), 3);
}
