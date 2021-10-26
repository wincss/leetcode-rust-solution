use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::next_greater_element(vv![4, 1, 2], vv![1, 3, 4, 2]),
        vv![-1, 3, -1]
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::next_greater_element(vv![2, 4], vv![1, 2, 3, 4]),
        vv![3, -1]
    );
}
