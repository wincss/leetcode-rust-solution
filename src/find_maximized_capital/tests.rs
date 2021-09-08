use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::find_maximized_capital(2, 0, vv![1, 2, 3], vv![0, 1, 1]),
        4
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::find_maximized_capital(3, 0, vv![1, 2, 3], vv![0, 1, 2]),
        6
    );
}
