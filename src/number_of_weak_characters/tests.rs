use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::number_of_weak_characters(vv![[5, 5], [6, 3], [3, 6]]),
        0
    );
}

#[test]
fn case_2() {
    assert_eq!(Solution::number_of_weak_characters(vv![[2, 2], [3, 3]]), 1);
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::number_of_weak_characters(vv![[1, 5], [10, 4], [4, 3]]),
        1
    );
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::number_of_weak_characters(vv![[1, 1], [2, 1], [2, 2], [1, 2]]),
        1
    );
}
