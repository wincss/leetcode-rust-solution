use crate::*;

#[test]
fn case_1() {
    assert_eq!(
        Solution::number_of_boomerangs(vv![[0, 0], [1, 0], [2, 0]]),
        2
    );
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::number_of_boomerangs(vv![[1, 1], [2, 2], [3, 3]]),
        2
    );
}

#[test]
fn case_3() {
    assert_eq!(Solution::number_of_boomerangs(vv![[1, 1]]), 0);
}
