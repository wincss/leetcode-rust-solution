use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::add_to_array_form(vec![1, 2, 0, 0], 34),
        vec![1, 2, 3, 4]
    );
}
#[test]
fn case_2() {
    assert_eq!(
        Solution::add_to_array_form(vec![2, 7, 4], 181),
        vec![4, 5, 5]
    );
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::add_to_array_form(vec![2, 1, 5], 806),
        vec![1, 0, 2, 1]
    );
}
#[test]
fn case_4() {
    assert_eq!(
        Solution::add_to_array_form(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}
