use crate::*;

#[test]
fn case_1() {
    assert_eq!(Solution::ways_to_step(3), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::ways_to_step(5), 13);
}

#[test]
fn case_3() {
    assert_eq!(Solution::ways_to_step(12234), 80003737);
}

#[test]
fn case_4() {
    assert_eq!(Solution::ways_to_step(1000000), 746580045);
}
