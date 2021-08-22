use crate::*;

#[test]
fn case_1() {
    assert!(Solution::escape_ghosts(vv![[1, 0], [0, 3]], vv![0, 1]));
}
#[test]
fn case_2() {
    assert!(!Solution::escape_ghosts(vv![[1, 0]], vv![2, 0]));
}

#[test]
fn case_3() {
    assert!(!Solution::escape_ghosts(vv![[2, 0]], vv![1, 0]));
}

#[test]
fn case_4() {
    assert!(!Solution::escape_ghosts(
        vv![[5, 0], [-10, -2], [0, -5], [-2, -2], [-7, 1]],
        vv![7, 7]
    ));
}
#[test]
fn case_5() {
    assert!(Solution::escape_ghosts(
        vv![[-1, 0], [0, 1], [-1, 0], [0, 1], [-1, 0]],
        vv![0, 0]
    ));
}

#[test]
fn case_6() {
    assert!(!Solution::escape_ghosts(
        vv![[1, 9], [2, -5], [3, 8], [9, 8], [-1, 3]],
        vv![8, -10]
    ));
}
