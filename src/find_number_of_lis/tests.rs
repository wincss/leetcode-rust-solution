use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::find_number_of_lis(vv![1, 3, 5, 4, 7]), 2);
}

#[test]
fn case_2() {
    assert_eq!(Solution::find_number_of_lis(vv![2, 2, 2, 2, 2]), 5);
}
