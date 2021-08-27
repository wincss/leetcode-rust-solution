use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::running_sum(vv![1, 2, 3, 4]), vv![1, 3, 6, 10]);
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::running_sum(vv![1, 1, 1, 1, 1]),
        vv![1, 2, 3, 4, 5]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::running_sum(vv![3, 1, 2, 10, 1]),
        vv![3, 4, 6, 16, 17]
    );
}
