use crate::*;

#[test]
fn case_1() {
    assert!(Solution::is_rectangle_cover(vv![
        [1, 1, 3, 3],
        [3, 1, 4, 2],
        [3, 2, 4, 4],
        [1, 3, 2, 4],
        [2, 3, 3, 4]
    ]));
}

#[test]
fn case_2() {
    assert!(!Solution::is_rectangle_cover(vv![
        [1, 1, 2, 3],
        [1, 3, 2, 4],
        [3, 1, 4, 2],
        [3, 2, 4, 4]
    ]));
}

#[test]
fn case_3() {
    assert!(!Solution::is_rectangle_cover(vv![
        [1, 1, 3, 3],
        [3, 1, 4, 2],
        [1, 3, 2, 4],
        [3, 2, 4, 4]
    ]));
}

#[test]
fn case_4() {
    assert!(!Solution::is_rectangle_cover(vv![
        [1, 1, 3, 3],
        [3, 1, 4, 2],
        [1, 3, 2, 4],
        [2, 2, 4, 4]
    ]));
}
